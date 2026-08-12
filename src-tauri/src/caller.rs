use tauri::{
    ipc::{CommandArg, CommandItem, InvokeError},
    Runtime, WebviewWindow,
};

const MAIN_LABEL: &str = "main";
const PET_LABEL: &str = "pet";

pub(crate) struct MainCaller;
pub(crate) struct PetCaller<R: Runtime = tauri::Wry>(pub(crate) WebviewWindow<R>);
pub(crate) struct RenderCaller<R: Runtime = tauri::Wry>(WebviewWindow<R>);

fn require_labels(
    webview_label: &str,
    window_label: &str,
    expected: &str,
) -> Result<(), InvokeError> {
    if webview_label == expected && window_label == expected {
        Ok(())
    } else {
        Err(InvokeError::from(
            "This command is not available from the current Memoryling surface.",
        ))
    }
}

impl<'de, R: Runtime> CommandArg<'de, R> for MainCaller {
    fn from_command(command: CommandItem<'de, R>) -> Result<Self, InvokeError> {
        let webview = command.message.webview_ref();
        require_labels(webview.label(), webview.window_ref().label(), MAIN_LABEL)?;
        Ok(Self)
    }
}

impl<'de, R: Runtime> CommandArg<'de, R> for PetCaller<R> {
    fn from_command(command: CommandItem<'de, R>) -> Result<Self, InvokeError> {
        let webview = command.message.webview_ref();
        require_labels(webview.label(), webview.window_ref().label(), PET_LABEL)?;
        let window = WebviewWindow::<R>::from_command(command)?;
        Ok(Self(window))
    }
}

impl<'de, R: Runtime> CommandArg<'de, R> for RenderCaller<R> {
    fn from_command(command: CommandItem<'de, R>) -> Result<Self, InvokeError> {
        let webview = command.message.webview_ref();
        require_labels(webview.label(), webview.window_ref().label(), PET_LABEL)?;
        let window = WebviewWindow::<R>::from_command(command)?;
        Ok(Self(window))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caller_labels_fail_closed() {
        assert!(require_labels(MAIN_LABEL, MAIN_LABEL, MAIN_LABEL).is_ok());
        assert!(require_labels(PET_LABEL, PET_LABEL, PET_LABEL).is_ok());
        assert!(require_labels(MAIN_LABEL, PET_LABEL, MAIN_LABEL).is_err());
        assert!(require_labels(PET_LABEL, MAIN_LABEL, MAIN_LABEL).is_err());
        assert!(require_labels(MAIN_LABEL, PET_LABEL, PET_LABEL).is_err());
        assert!(require_labels(PET_LABEL, MAIN_LABEL, PET_LABEL).is_err());
        assert!(require_labels("unknown", "unknown", MAIN_LABEL).is_err());
        assert!(require_labels("unknown", "unknown", PET_LABEL).is_err());
    }
}
