use crate::app::App;
use crate::kakisute_file::KakisuteFile;
use crate::ui::tui_app::Mode;
use crate::ui::tui_app::Tui;

const DELETE_MODAL_BODY: &str = "Are you sure you want to delete? (Y/n)";
const DELETE_MODAL_TITLE: &str = "Confirm Modal";
const KAKISUTE_LIST_TITLE: &str = "List";
const NO_FILE_BODY: &str = "<No file is selected>";
const CONTENT_TITLE: &str = "Content";
const NEW_FILE_NAME_MODAL_TITLE: &str = "Input new file name";
const HELP_NORMAL_BODY: &str =
    "esc: Quit, j: Down, k: Up, e: Edit, n: Create new, N: Create new with file name, d: Delete";

const HELP_INSERT_BODY: &str = "esc: Enter normal mode, Enter: Open editor";
const HELP_DELETE_BODY: &str = "esc/n: Cancel, Y: delete";
const HELP_TITLE: &str = "Help";

pub struct DisplayData<'a> {
    pub index: Option<usize>,
    pub mode: &'a Mode,
    pub kakisute_list: BlockData<&'a [KakisuteFile]>,
    pub content: BlockData<String>,
    pub new_file_name_modal: BlockData<&'a str>,
    pub help: BlockData<String>,
    pub delete_modal: BlockData<&'a str>,
}

impl<'a> DisplayData<'a> {
    pub fn new(app: &App, tui: &'a Tui) -> Self {
        let kakisute_list = BlockData::new(tui.items.as_ref(), KAKISUTE_LIST_TITLE);

        let kakisute_content = tui.get_selected_kakisute_content(app);

        let content = DisplayData::create_content(kakisute_content);

        let new_file_name = DisplayData::create_new_file_name_modal(tui.new_file_name.as_str());

        let help = DisplayData::create_help(&tui.mode);

        let delete_modal = BlockData::new(DELETE_MODAL_BODY, DELETE_MODAL_TITLE);

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
            None => NO_FILE_BODY.to_string(),
        };
        BlockData::new(content_body, CONTENT_TITLE)
    }

    fn create_new_file_name_modal(new_file_name: &'a str) -> BlockData<&'a str> {
        BlockData::new(new_file_name, NEW_FILE_NAME_MODAL_TITLE)
    }

    fn create_help(mode: &Mode) -> BlockData<String> {
        let help_body = match mode {
            Mode::Normal => HELP_NORMAL_BODY,
            Mode::Insert => HELP_INSERT_BODY,
            Mode::DeleteConfirm => HELP_DELETE_BODY,
        }
        .to_string();
        BlockData::new(help_body, HELP_TITLE)
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

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    describe "create_help" {
        it "returns Normal help when Normal mode" {
            let res = DisplayData::create_help(&Mode::Normal);
            assert_eq!(res.body, HELP_NORMAL_BODY)
        }

        it "returns Insert help when Insert mode" {
            let res = DisplayData::create_help(&Mode::Insert);
            assert_eq!(res.body, HELP_INSERT_BODY)
        }

        it "returns Delete help when Delete mode" {
            let res = DisplayData::create_help(&Mode::DeleteConfirm);
            assert_eq!(res.body, HELP_DELETE_BODY)
        }
    }

    describe "create_content" {
        it "return conetent when exist" {
            let res = DisplayData::create_content(Some("content".to_string()));
            assert_eq!(res.body, "content")
        }

        it "return empty message when not exist" {
            let res = DisplayData::create_content(None);
            assert_eq!(res.body, NO_FILE_BODY)
        }
    }
}
