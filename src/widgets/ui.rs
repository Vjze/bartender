use iced::widget::image::Handle;
use iced::widget::{horizontal_space, image, pick_list};
use iced::{
    executor, theme,
    widget::{button, column, container, row, text, text_input, tooltip},
    Alignment, Application, Command, Element, Length, Theme,
};
use iced_aw::{number_input, NumberInputStyles};

use crate::logic::init::{init_all, sql_init, InitData};
use crate::logic::print::{print, updata};
use crate::logic::sql_select::run_query;
use crate::resource::assets::fonts::FONT;
use crate::resource::assets::pngs::{C_LOGO, RIGHT, WAIT, WRONGS};
use crate::resource::r#enum::{Btw, DataInfo, Error, Printers, SqlSelect};

use super::tip::{loading_message, tip};

pub enum Bartender {
    Loading,
    Loaded(State),
}
#[derive(Debug, Default)]
pub struct State {
    sqlpath: SqlSelect,
    sql: String,
    printers: Vec<Printers>,
    printer: Option<Printers>,
    input: String,
    pub model: bool,
    ret: u32,
    librarie: String,
    list: Vec<DataInfo>,
    status: String,
    tips: Error,
    btws: Vec<Btw>,
    btw: Option<Btw>,
}
#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<InitData, Error>),
    FontLoaded(Result<(), iced::font::Error>),
    InputChange(String),
    Print,
    SqlSelect(SqlSelect),
    PrintSelect(Printers),
    NumInpChanged(u32),
    PrintBack(Result<bool, Error>),
    SqlResultBack(Result<Vec<DataInfo>, Error>),
    Updata(Result<String, Error>),
    OpenModal,
    CloseModal,
    BtwSelect(Btw),
    SqlInit(Result<String, Error>),
    Nothing,
}
impl Application for Bartender {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Bartender, iced::Command<Self::Message>) {
        let font_command = iced::font::load(FONT);
        let sqlpath = SqlSelect::default();
        (
            Bartender::Loading,
            Command::batch([
                font_command.map(Message::FontLoaded),
                Command::perform(init_all(), Message::Loaded),
                Command::perform(sql_init(sqlpath.to_string()), Message::SqlInit),
            ]),
        )
    }

    fn title(&self) -> String {
        format!(
            "{} - V {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match self {
            Bartender::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Bartender::Loaded(State {
                            printers: state.printers,
                            librarie: state.librarie_id,
                            btws: state.btws,
                            status: "空闲中".to_string(),
                            ..State::default()
                        })
                    }
                    Message::Loaded(Err(e)) => {
                        *self = Bartender::Loaded(State {
                            model: true,
                            tips: e,
                            status: "错误！！！！".to_string(),
                            ..State::default()
                        })
                    }
                    _ => {}
                }
                Command::none()
            }
            Bartender::Loaded(state) => {
                let command = match message {
                    Message::Loaded(_b) => Command::none(),
                    Message::FontLoaded(_) => Command::none(),
                    Message::InputChange(s) => {
                        state.input = s;
                        Command::none()
                    }
                    Message::Print => {
                        let sn = state.input.clone();
                        let sql = state.sqlpath.clone();
                        state.status = "打印中...".to_string();
                        state.input.clear();
                        Command::perform(run_query(sn, sql.to_string()), Message::SqlResultBack)
                    }
                    Message::SqlSelect(sql) => {
                        state.sqlpath = sql;
                        Command::perform(
                            sql_init(state.sqlpath.clone().to_string()),
                            Message::SqlInit,
                        )
                    }
                    Message::PrintSelect(printer) => {
                        state.printer = Some(printer);
                        Command::none()
                    }
                    Message::NumInpChanged(u) => {
                        state.ret = u;
                        Command::none()
                    }
                    Message::PrintBack(b) => match b {
                        Ok(b) => match b {
                            true => {
                                let list = state.list.clone();
                                let sql = state.sqlpath.clone();
                                Command::perform(updata(list, sql.to_string()), Message::Updata)
                            }
                            false => {
                                state.status = "空闲中...".to_string();
                                Command::none()
                            }
                        },
                        Err(e) => {
                            state.tips = e;
                            state.status = "空闲中...".to_string();
                            Command::none()
                        }
                    },
                    Message::SqlResultBack(res) => match res {
                        Ok(r) => {
                            state.list = r;
                            let librarie = state.librarie.clone();
                            let list = state.list.clone();
                            let printers = state.printer.clone();
                            let printer = match printers {
                                Some(p) => Ok(p),
                                None => Err(Error::PrinterNoSelect),
                            };
                            let float = state.ret.clone();
                            let btw = match state.btw.clone() {
                                Some(b) => Ok(b),
                                None => Err(Error::BtwNoSelect),
                            };
                            match btw {
                                Ok(btw) => match printer {
                                    Ok(p) => Command::perform(
                                        print(list, librarie, btw.btw, p.printer, float),
                                        Message::PrintBack,
                                    ),
                                    Err(e) => {
                                        state.model = true;
                                        state.tips = e;
                                        state.status = "空闲中...".to_string();
                                        Command::none()
                                    }
                                },
                                Err(e) => {
                                    state.model = true;
                                    state.tips = e;
                                    state.status = "空闲中...".to_string();
                                    Command::none()
                                }
                            }
                        }
                        Err(e) => {
                            state.model = true;
                            state.tips = e;
                            state.status = "空闲中...".to_string();
                            Command::none()
                        }
                    },
                    Message::Updata(s) => {
                        match s {
                            Ok(s) => state.status = s,
                            Err(e) => {
                                state.tips = e;
                                state.model = true;
                                state.status = "空闲中...".to_string();
                            }
                        };
                        Command::none()
                    }
                    Message::OpenModal => {
                        state.model = true;
                        Command::none()
                    }
                    Message::CloseModal => {
                        state.model = false;
                        Command::none()
                    }
                    Message::BtwSelect(btw) => {
                        state.btw = Some(btw);
                        Command::none()
                    }
                    Message::SqlInit(s) => {
                        match s {
                            Ok(s) => {
                                state.sql = s;
                            }
                            Err(e) => {
                                state.tips = e;
                                state.sql = "错误".to_string();
                                state.model = true;
                                state.status = "空闲中...".to_string();
                            }
                        };
                        Command::none()
                    }
                    Message::Nothing => Command::none(),
                };
                Command::batch(vec![command])
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self {
            Bartender::Loading => loading_message(),
            Bartender::Loaded(State {
                input,
                printers,
                printer,
                model,
                sql,
                sqlpath,
                status,
                btws,
                btw,
                tips,
                ret,
                ..
            }) => {
                let c_logo = image(Handle::from_memory(C_LOGO)).width(Length::Fixed(200.0));
                let input = text_input("扫描条码....", &input)
                    .on_input(Message::InputChange)
                    .on_submit(
                        (!printers.is_empty() && !model == true && sql == "成功")
                            .then_some(Message::Print)
                            .unwrap_or(Message::Nothing),
                    );
                let print_btn = action(
                    text("打印"),
                    "点击开始打印",
                    (!printers.is_empty() && !model == true && sql == "成功")
                        .then_some(Message::Print),
                );
                let sql_text = text("选择数据库:");
                let sql_select = pick_list(
                    &SqlSelect::ALL[..],
                    Some(sqlpath.clone()),
                    Message::SqlSelect,
                );
                let sql_t = if *sql == "成功" {
                    image(Handle::from_memory(RIGHT)).width(Length::Fixed(20.0))
                } else if *sql == "错误" {
                    image(Handle::from_memory(WRONGS)).width(Length::Fixed(20.0))
                } else {
                    image(Handle::from_memory(WAIT)).width(Length::Fixed(20.0))
                };
                let sql_status = sql_t;
                let first_row =
                    row!(input, sql_text, sql_select, sql_status, print_btn).spacing(25);
                let ret_text = text("选择RET小数位:");
                let ret_int = number_input(*ret, 10, Message::NumInpChanged)
                    .style(NumberInputStyles::Default)
                    .step(1);

                let btw_text = text("选择模板:");
                let btw_select = pick_list(&*btws, btw.clone(), Message::BtwSelect);
                let btw_status = if btws.is_empty() {
                    image(Handle::from_memory(WRONGS)).width(Length::Fixed(20.0))
                } else if btw.is_none() {
                    image(Handle::from_memory(WAIT)).width(Length::Fixed(20.0))
                } else {
                    image(Handle::from_memory(RIGHT)).width(Length::Fixed(20.0))
                };

                let print = text("选择打印机:");
                let printers_select = pick_list(&*printers, printer.clone(), Message::PrintSelect);
                let printer_status = if printers.is_empty() {
                    image(Handle::from_memory(WRONGS)).width(Length::Fixed(20.0))
                } else if printer.is_none() {
                    image(Handle::from_memory(WAIT)).width(Length::Fixed(20.0))
                } else {
                    image(Handle::from_memory(RIGHT)).width(Length::Fixed(20.0))
                };
                let sen_row = row!(
                    ret_text,
                    ret_int,
                    horizontal_space(Length::Fill),
                    btw_text,
                    btw_select,
                    btw_status,
                    print,
                    printers_select,
                    printer_status
                )
                .spacing(15)
                .align_items(Alignment::Center)
                .width(Length::Fill);
                let status = text(status.clone()).size(25);
                let col = column!(c_logo, first_row, sen_row, status)
                    .spacing(20)
                    .align_items(Alignment::Center);
                let body = match tips {
                    Error::Inputisempty => "输入框不能为空,请输入后再进行打印!!!",
                    Error::SnNone => "未找到SN,请确认SN是否正确!!!",
                    Error::NoFirst => "SN非第一次打印,请确认SN是否正确!!!",
                    Error::SqlIDErr => "数据库连接失败,请确认数据库连接是否正常!!!",
                    Error::LibrariesNone => {
                        "未找到标签模板库,请确认网络环境是否正常,请确认电脑上是否安装Bartender,并且配置完成!!!"
                    }
                    Error::UpdataErr => "数据无法写入数据库,请检查数据库连接是否正常",
                    Error::PrintErr => {
                        "标签打印错误,请确认网络环境是否正常,电脑上是否安装Bartender,并且模板名称是否正确!!!"
                    }
                    Error::PrinterErr => {
                        "未找到打印机,请确认网络环境是否正常,请确认电脑上是否安装Bartender,并且配置完成!!!"
                    }
                    Error::BtwErr => "未找到标签模板,请确认网络环境是否正常,请确认电脑上是否安装Bartender,并且配置完成!!!",
                    Error::BtwNoSelect => "未选择标签模板,请先选择模板再进行打印!!!",
                    Error::PrinterNoSelect => "未选择打印机,请先选择打印机再进行打印!!!",
                    Error::InitNotDone => "软件自检未通过,请检查数据库是否正常连接....",
                    Error::NetWrongs => "网络异常！！！！",
                };
                let all_tip = tip(*model, container(col), body);
                container(all_tip).padding(15).into()
            }
        }
    }
}

fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).width(50).center_x());

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(theme::Container::Box)
        .into()
    } else {
        action.style(theme::Button::Secondary).into()
    }
}
