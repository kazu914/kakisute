use crate::app::App;
use crate::kakisute_file::KakisuteFile;
use crate::ui::tui_app::Mode;
use crate::ui::tui_app::Tui;

pub struct DisplayData<'a> {
    pub index: Option<usize>,
    pub mode: &'a Mode,
    pub kakisute_list: BlockData<&'a [KakisuteFile]>,
    pub content: BlockData<String>,
    pub new_file_name_modal: BlockData<&'a str>,
    pub help: BlockData<String>,
    pub delete_modal: BlockData<String>,
}

impl<'a> DisplayData<'a> {
    pub fn new(app: &App, tui: &'a Tui) -> Self {
        let kakisute_list = BlockData::new(tui.items.as_ref(), " List");

        let kakisute_content = tui.get_selected_kakisute_content(app);

        let content = DisplayData::create_content(kakisute_content);

        let new_file_name = DisplayData::create_new_file_name_modal(tui.new_file_name.as_str());

        let help = DisplayData::create_help(&tui.mode);

        let delete_modal = BlockData::new(
            "Are you sure you want to delete? (Y/n)".to_string(),
            "Confirm Modal",
        );

        Self {
            index: tui.selected_list_index,
            mode: &tui.mode,
            kakisute_list,
            content,
            new_file_name_modal: new_file_name,
            help,
            delete_modal,
        }
    }

    fn create_content(kakisute_content: Option<String>) -> BlockData<String> {
        let content_body = match kakisute_content {
            Some(kakisute_content) => kakisute_content,
            None => "<No file is selected>".to_string(),
        };
        BlockData::new(content_body, "Content")
    }

    fn create_new_file_name_modal(new_file_name: &'a str) -> BlockData<&'a str> {
        BlockData::new(new_file_name, "Input new file name")
    }

    fn create_help(mode: &Mode) -> BlockData<String> {
        let help_body = match mode {
            Mode::Normal => {
                "esc: Quit, j: Down, k: Up, e: Edit, n: Create new, N: Create new with file name, d: Delete"
            }
            Mode::Insert => "esc: Enter normal mode, Enter: Open editor",
            Mode::DeleteConfirm => "esc/n: Cancel, Y: delete",
        }.to_string();
        BlockData::new(help_body, "Help")
    }
}

pub struct BlockData<T> {
    pub body: T,
    pub title: String,
}

impl<T> BlockData<T> {
    fn new(body: T, title: &str) -> Self {
        Self {
            body,
            title: title.to_string(),
        }
    }
}
