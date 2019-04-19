use std::env::args_os;
use std::ffi::{OsStr, OsString};
use std::rc::Rc;

use super::{intercept_global_installs, ToolCommand};
use crate::error::ErrorDetails;
use crate::platform::PlatformSpec;
use crate::session::{ActivityKind, Session};

use either::{Either, Left, Right};
use notion_fail::{throw, Fallible};

pub(super) fn yarn_command<A>(args: A, session: &mut Session) -> Fallible<ToolCommand>
where
    A: IntoIterator<Item = OsString>,
{
    session.add_event_start(ActivityKind::Yarn);

    match get_yarn_platform(session)? {
        Left(ref platform) => {
            if intercept_global_installs() && is_global_yarn_add() {
                throw!(ErrorDetails::NoGlobalInstalls);
            }

            let image = platform.checkout(session)?;
            let path = image.path()?;
            Ok(ToolCommand::direct(OsStr::new("yarn"), args, &path))
        }
        Right(error) => ToolCommand::passthrough(OsStr::new("yarn"), args, error),
    }
}

/// Determine the correct platform (project or user) and check if yarn is set for that platform
fn get_yarn_platform(session: &mut Session) -> Fallible<Either<Rc<PlatformSpec>, ErrorDetails>> {
    // First check if we are in a pinned project
    if let Some(platform) = session.project_platform()? {
        return match platform.yarn {
            Some(_) => Ok(Left(platform)),
            None => Ok(Right(ErrorDetails::NoProjectYarn)),
        };
    }

    // If not, fall back to the user platform
    if let Some(platform) = session.user_platform()? {
        return match platform.yarn {
            Some(_) => Ok(Left(platform)),
            None => Ok(Right(ErrorDetails::NoUserYarn)),
        };
    }

    Ok(Right(ErrorDetails::NoPlatform))
}

fn is_global_yarn_add() -> bool {
    // Yarn global installs must be of the form `yarn global add`
    // However, they may have options intermixed, e.g. yarn --verbose global add ember-cli
    args_os()
        .skip(1)
        .filter(|arg| match arg.to_str() {
            Some(arg) => !arg.starts_with("-"),
            None => true,
        })
        .take(2)
        .eq(vec!["global", "add"])
}
