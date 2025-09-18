//! Verbatim copy-paste of security_framework `Authorization` but missing the flags field
//! therefore I can grab an unsafe reference to it.

#![cfg(target_os = "macos")]

use core_foundation::base::{CFTypeRef, OSStatus, TCFType};
use core_foundation::bundle::CFBundleRef;
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::CFString;
use security_framework::authorization::{AuthorizationItemSetStorage, Flags, RightDefinition};
use security_framework::base::{Error, Result as SecResult};
use security_framework_sys::authorization::{
    errAuthorizationSuccess, AuthorizationCopyInfo, AuthorizationCreate,
    AuthorizationCreateFromExternalForm, AuthorizationExecuteWithPrivileges,
    AuthorizationExternalForm, AuthorizationFree, AuthorizationFreeItemSet,
    AuthorizationItemSet as sys_AuthorizationItemSet, AuthorizationMakeExternalForm,
    AuthorizationRef, AuthorizationRightGet, AuthorizationRightRemove, AuthorizationRightSet,
};
use security_framework_sys::base::{errSecConversionError, errSecSuccess};
use std::ffi::CString;
use std::fs::File;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::addr_of;

macro_rules! optional_str_to_cfref {
    ($string:ident) => {{
        $string
            .map(CFString::new)
            .map_or(std::ptr::null(), |cfs| cfs.as_concrete_TypeRef())
    }};
}

macro_rules! cstring_or_err {
    ($x:expr) => {{
        CString::new($x).map_err(|_| Error::from_code(errSecConversionError))
    }};
}

fn cvt(err: OSStatus) -> SecResult<()> {
    match err {
        errSecSuccess => Ok(()),
        err => Err(Error::from_code(err)),
    }
}

/// A set of authorization items returned and owned by the Security Server.
#[derive(Debug)]
#[repr(C)]
pub struct AuthorizationItemSet<'a> {
    inner: *const sys_AuthorizationItemSet,
    phantom: PhantomData<&'a sys_AuthorizationItemSet>,
}

impl Drop for AuthorizationItemSet<'_> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            AuthorizationFreeItemSet(self.inner.cast_mut());
        }
    }
}

#[derive(Debug)]
pub struct SimpleAuthorization {
    handle: AuthorizationRef,
}

impl TryFrom<AuthorizationExternalForm> for SimpleAuthorization {
    type Error = Error;

    /// Internalizes the external representation of an authorization reference.
    #[cold]
    fn try_from(external_form: AuthorizationExternalForm) -> SecResult<Self> {
        let mut handle = MaybeUninit::<AuthorizationRef>::uninit();

        let status =
            unsafe { AuthorizationCreateFromExternalForm(&external_form, handle.as_mut_ptr()) };

        if status != errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        let auth = Self {
            handle: unsafe { handle.assume_init() },
        };

        Ok(auth)
    }
}

impl SimpleAuthorization {
    pub fn get_ref(&self) -> AuthorizationRef {
        self.handle
    }

    /// Creates an authorization object which has no environment or associated
    /// rights.
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> SecResult<Self> {
        Self::new(None, None, Default::default())
    }

    /// Creates an authorization reference and provides an option to authorize
    /// or preauthorize rights.
    ///
    /// `rights` should be the names of the rights you want to create.
    ///
    /// `environment` is used when authorizing or preauthorizing rights. Not
    /// used in OS X v10.2 and earlier. In macOS 10.3 and later, you can pass
    /// icon or prompt data to be used in the authentication dialog box. In
    /// macOS 10.4 and later, you can also pass a user name and password in
    /// order to authorize a user without user interaction.
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        // FIXME: this should have been by reference
        rights: Option<AuthorizationItemSetStorage>,
        environment: Option<AuthorizationItemSetStorage>,
        flags: Flags,
    ) -> SecResult<Self> {
        let rights_ptr = rights.as_ref().map_or(std::ptr::null(), |r| {
            addr_of!(r.set).cast::<sys_AuthorizationItemSet>()
        });

        let env_ptr = environment.as_ref().map_or(std::ptr::null(), |e| {
            addr_of!(e.set).cast::<sys_AuthorizationItemSet>()
        });

        let mut handle = MaybeUninit::<AuthorizationRef>::uninit();

        let status =
            unsafe { AuthorizationCreate(rights_ptr, env_ptr, flags.bits(), handle.as_mut_ptr()) };

        if status != errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        Ok(Self {
            handle: unsafe { handle.assume_init() },
        })
    }

    /// Internalizes the external representation of an authorization reference.
    #[deprecated(since = "2.0.1", note = "Please use the TryFrom trait instead")]
    pub fn from_external_form(external_form: AuthorizationExternalForm) -> SecResult<Self> {
        external_form.try_into()
    }

    /// Retrieve's the right's definition as a dictionary. Use `right_exists`
    /// if you want to avoid retrieving the dictionary.
    ///
    /// `name` can be a wildcard right name.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    // TODO: deprecate and remove. CFDictionary should not be exposed in public Rust APIs.
    pub fn get_right<T: Into<Vec<u8>>>(name: T) -> SecResult<CFDictionary<CFString, CFTypeRef>> {
        let name = cstring_or_err!(name)?;
        let mut dict = MaybeUninit::<CFDictionaryRef>::uninit();

        let status = unsafe { AuthorizationRightGet(name.as_ptr(), dict.as_mut_ptr()) };

        if status != errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        let dict = unsafe { CFDictionary::wrap_under_create_rule(dict.assume_init()) };

        Ok(dict)
    }

    /// Checks if a right exists within the policy database. This is the same as
    /// `get_right`, but avoids a dictionary allocation.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn right_exists<T: Into<Vec<u8>>>(name: T) -> SecResult<bool> {
        let name = cstring_or_err!(name)?;

        let status = unsafe { AuthorizationRightGet(name.as_ptr(), std::ptr::null_mut()) };

        Ok(status == errAuthorizationSuccess)
    }

    /// Removes a right from the policy database.
    ///
    /// `name` cannot be a wildcard right name.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn remove_right<T: Into<Vec<u8>>>(&self, name: T) -> SecResult<()> {
        let name = cstring_or_err!(name)?;

        let status = unsafe { AuthorizationRightRemove(self.handle, name.as_ptr()) };

        if status != errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        Ok(())
    }

    /// Creates or updates a right entry in the policy database. Your process
    /// must have a code signature in order to be able to add rights to the
    /// authorization database.
    ///
    /// `name` cannot be a wildcard right.
    ///
    /// `definition` can be either a `CFDictionaryRef` containing keys defining
    /// the rules or a `CFStringRef` representing the name of another right
    /// whose rules you wish to duplicaate.
    ///
    /// `description` is a key which can be used to look up localized
    /// descriptions.
    ///
    /// `bundle` will be used to get localizations from if not the main bundle.
    ///
    /// `localeTableName` will be used to get localizations if provided.
    ///
    /// If `name` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn set_right<T: Into<Vec<u8>>>(
        &self,
        name: T,
        definition: RightDefinition<'_>,
        description: Option<&str>,
        bundle: Option<CFBundleRef>,
        locale: Option<&str>,
    ) -> SecResult<()> {
        let name = cstring_or_err!(name)?;

        let definition_cfstring: CFString;
        let definition_ref = match definition {
            RightDefinition::FromDictionary(def) => def.as_CFTypeRef(),
            RightDefinition::FromExistingRight(def) => {
                definition_cfstring = CFString::new(def);
                definition_cfstring.as_CFTypeRef()
            }
        };

        let status = unsafe {
            AuthorizationRightSet(
                self.handle,
                name.as_ptr(),
                definition_ref,
                optional_str_to_cfref!(description),
                bundle.unwrap_or(std::ptr::null_mut()),
                optional_str_to_cfref!(locale),
            )
        };

        if status != errAuthorizationSuccess {
            return Err(Error::from_code(status));
        }

        Ok(())
    }

    /// An authorization plugin can store the results of an authentication
    /// operation by calling the `SetContextValue` function. You can then
    /// retrieve this supporting data, such as the user name.
    ///
    /// `tag` should specify the type of data the Security Server should return.
    /// If `None`, all available information is retreieved.
    ///
    /// If `tag` isn't convertable to a `CString` it will return
    /// Err(errSecConversionError).
    pub fn copy_info<T: Into<Vec<u8>>>(
        &self,
        tag: Option<T>,
    ) -> SecResult<AuthorizationItemSet<'_>> {
        let tag_with_nul: CString;

        let tag_ptr = match tag {
            Some(tag) => {
                tag_with_nul = cstring_or_err!(tag)?;
                tag_with_nul.as_ptr()
            }
            None => std::ptr::null(),
        };

        let mut inner = MaybeUninit::<*mut sys_AuthorizationItemSet>::uninit();

        let status = unsafe { AuthorizationCopyInfo(self.handle, tag_ptr, inner.as_mut_ptr()) };

        if status != errAuthorizationSuccess {
            return Err(Error::from(status));
        }

        let set = AuthorizationItemSet {
            inner: unsafe { inner.assume_init() },
            phantom: PhantomData,
        };

        Ok(set)
    }

    /// Creates an external representation of an authorization reference so that
    /// you can transmit it between processes.
    pub fn make_external_form(&self) -> SecResult<AuthorizationExternalForm> {
        let mut external_form = MaybeUninit::<AuthorizationExternalForm>::uninit();

        let status =
            unsafe { AuthorizationMakeExternalForm(self.handle, external_form.as_mut_ptr()) };

        if status != errAuthorizationSuccess {
            return Err(Error::from(status));
        }

        Ok(unsafe { external_form.assume_init() })
    }

    /// Runs an executable tool with root privileges.
    /// Discards executable's output
    #[cfg(target_os = "macos")]
    #[inline(always)]
    pub fn execute_with_privileges<P, S, I>(
        &self,
        command: P,
        arguments: I,
        flags: Flags,
    ) -> SecResult<()>
    where
        P: AsRef<std::path::Path>,
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        use std::os::unix::ffi::OsStrExt;

        let arguments = arguments
            .into_iter()
            .flat_map(|a| CString::new(a.as_ref().as_bytes()))
            .collect::<Vec<_>>();
        self.execute_with_privileges_internal(
            command.as_ref().as_os_str().as_bytes(),
            &arguments,
            flags,
            false,
        )?;
        Ok(())
    }

    /// Runs an executable tool with root privileges,
    /// and returns a `File` handle to its communication pipe
    #[cfg(target_os = "macos")]
    #[inline(always)]
    pub fn execute_with_privileges_piped<P, S, I>(
        &self,
        command: P,
        arguments: I,
        flags: Flags,
    ) -> SecResult<File>
    where
        P: AsRef<std::path::Path>,
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        use std::os::unix::ffi::OsStrExt;

        let arguments = arguments
            .into_iter()
            .flat_map(|a| CString::new(a.as_ref().as_bytes()))
            .collect::<Vec<_>>();
        Ok(self
            .execute_with_privileges_internal(
                command.as_ref().as_os_str().as_bytes(),
                &arguments,
                flags,
                true,
            )?
            .unwrap())
    }

    // Runs an executable tool with root privileges.
    #[cfg(target_os = "macos")]
    fn execute_with_privileges_internal(
        &self,
        command: &[u8],
        arguments: &[CString],
        flags: Flags,
        make_pipe: bool,
    ) -> SecResult<Option<File>> {
        use std::os::unix::io::{FromRawFd, RawFd};

        let c_cmd = cstring_or_err!(command)?;

        let mut c_args = arguments
            .iter()
            .map(|a| a.as_ptr().cast_mut())
            .collect::<Vec<_>>();
        c_args.push(std::ptr::null_mut());

        let mut pipe: *mut libc::FILE = std::ptr::null_mut();

        let status = unsafe {
            AuthorizationExecuteWithPrivileges(
                self.handle,
                c_cmd.as_ptr(),
                flags.bits(),
                c_args.as_ptr(),
                if make_pipe {
                    &mut pipe
                } else {
                    std::ptr::null_mut()
                },
            )
        };

        cvt(status)?;
        Ok(if make_pipe {
            if pipe.is_null() {
                return Err(Error::from_code(32)); // EPIPE?
            }
            Some(unsafe { File::from_raw_fd(libc::fileno(pipe) as RawFd) })
        } else {
            None
        })
    }
}

impl Drop for SimpleAuthorization {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            AuthorizationFree(self.handle, Flags::default().bits());
        }
    }
}
