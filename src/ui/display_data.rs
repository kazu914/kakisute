use crate::ui::app_interactor::Mode;

const DELETE_MODAL_BODY: &str = "Are you sure you want to delete? (Y/n)";
const DELETE_MODAL_TITLE: &str = "Confirm Modal";
const KAKISUTE_LIST_TITLE: &str = "List";
const NO_FILE_BODY: &str = "<No file is selected>";
const CONTENT_TITLE: &str = "Content";
const NEW_FILE_NAME_MODAL_TITLE: &str = "Input new file name";
const SEARCH_MODAL_TITLE: &str = "Input search query";
const HELP_NORMAL_BODY: &str =
    "esc/q: Quit, j: Down, k: Up, ^d: Down 1/2 screen, ^u: Up 1/2 screen, e: Edit, n: Create new, N: Create new with file name, d: Delete";

const HELP_INSERT_BODY: &str = "esc: Enter normal mode, Enter: Open editor";
const HELP_DELETE_BODY: &str = "esc/n: Cancel, Y: delete";
const HELP_SEARCH_BODY: &str = "esc/n: Cancel, Y: delete";
const HELP_TITLE: &str = "Help";

pub struct DisplayData<'a> {
    pub index: Option<usize>,
    pub mode: Mode,
    pub kakisute_list: BlockData<Vec<&'a str>>,
    pub content: BlockData<String>,
    pub new_filename: BlockData<String>,
    pub search_query: BlockData<String>,
    pub help: BlockData<String>,
    pub delete_modal: BlockData<&'a str>,
    pub need_search_box: bool,
}

pub struct Info<'a> {
    pub index: Option<usize>,
    pub mode: Mode,
    pub kakisute_list: Vec<&'a str>,
    pub content: Option<String>,
    pub new_filename: String,
    pub search_query: String,
}

impl<'a> DisplayData<'a> {
    pub fn new(info: Info<'a>) -> Self {
        let kakisute_list = DisplayData::create_kakisute_list(info.kakisute_list, info.index);

        let content = DisplayData::create_content(info.content);

        let new_filename = DisplayData::create_new_filename_modal(&info.new_filename);

        let search_query = DisplayData::create_search_query_modal(&info.search_query);

        let help = DisplayData::create_help(&info.mode);

        let delete_modal = BlockData::new(DELETE_MODAL_BODY, DELETE_MODAL_TITLE);

        Self {
            index: info.index,
            mode: info.mode,
            kakisute_list,
            content,
            new_filename,
            search_query,
            help,
            delete_modal,
            need_search_box: !info.search_query.is_empty() || info.mode == Mode::Search,
        }
    }

    fn create_kakisute_list(
        kakisute_list: Vec<&'a str>,
        index: Option<usize>,
    ) -> BlockData<Vec<&'a str>> {
        let title = if let Some(index) = index {
            format!(
                "{} ({}/{})",
                KAKISUTE_LIST_TITLE.to_owned(),
                &(index + 1).to_string(),
                &kakisute_list.len().to_string()
            )
        } else {
            KAKISUTE_LIST_TITLE.to_string()
        };
        BlockData::new(kakisute_list, &title)
    }

    fn create_content(kakisute_content: Option<String>) -> BlockData<String> {
        let content_body = match kakisute_content {
            Some(kakisute_content) => kakisute_content,
            None => NO_FILE_BODY.to_string(),
        };
        BlockData::new(content_body, CONTENT_TITLE)
    }

    fn create_new_filename_modal(user_input: &str) -> BlockData<String> {
        BlockData::new(user_input.to_string(), NEW_FILE_NAME_MODAL_TITLE)
    }

    fn create_search_query_modal(user_input: &str) -> BlockData<String> {
        BlockData::new(user_input.to_string(), SEARCH_MODAL_TITLE)
    }

    fn create_help(mode: &Mode) -> BlockData<String> {
        let help_body = match mode {
            Mode::Normal => HELP_NORMAL_BODY,
            Mode::Insert => HELP_INSERT_BODY,
            Mode::DeleteConfirm => HELP_DELETE_BODY,
            Mode::Search => HELP_SEARCH_BODY,
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
        it "return content when exist" {
            let res = DisplayData::create_content(Some("content".to_string()));
            assert_eq!(res.body, "content")
        }

        it "return empty message when not exist" {
            let res = DisplayData::create_content(None);
            assert_eq!(res.body, NO_FILE_BODY)
        }
    }
}
