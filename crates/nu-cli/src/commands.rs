#[macro_use]
pub(crate) mod macros;

mod from_delimited_data;
mod to_delimited_data;

pub(crate) mod append;
pub(crate) mod args;
pub(crate) mod autoview;
pub(crate) mod calc;
pub(crate) mod cd;
pub(crate) mod classified;
pub(crate) mod clip;
pub(crate) mod command;
pub(crate) mod compact;
pub(crate) mod config;
pub(crate) mod count;
pub(crate) mod cp;
pub(crate) mod date;
pub(crate) mod debug;
pub(crate) mod default;
pub(crate) mod du;
pub(crate) mod each;
pub(crate) mod echo;
pub(crate) mod edit;
pub(crate) mod enter;
#[allow(unused)]
pub(crate) mod evaluate_by;
pub(crate) mod exit;
pub(crate) mod first;
pub(crate) mod format;
pub(crate) mod from_bson;
pub(crate) mod from_csv;
pub(crate) mod from_ics;
pub(crate) mod from_ini;
pub(crate) mod from_json;
pub(crate) mod from_ods;
pub(crate) mod from_sqlite;
pub(crate) mod from_ssv;
pub(crate) mod from_toml;
pub(crate) mod from_tsv;
pub(crate) mod from_url;
pub(crate) mod from_vcf;
pub(crate) mod from_xlsx;
pub(crate) mod from_xml;
pub(crate) mod from_yaml;
pub(crate) mod get;
pub(crate) mod group_by;
pub(crate) mod headers;
pub(crate) mod help;
pub(crate) mod histogram;
pub(crate) mod history;
pub(crate) mod insert;
pub(crate) mod last;
pub(crate) mod lines;
pub(crate) mod ls;
#[allow(unused)]
pub(crate) mod map_max_by;
pub(crate) mod mkdir;
pub(crate) mod mv;
pub(crate) mod next;
pub(crate) mod nth;
pub(crate) mod open;
pub(crate) mod parse;
pub(crate) mod pick;
pub(crate) mod pivot;
pub(crate) mod plugin;
pub(crate) mod prepend;
pub(crate) mod prev;
pub(crate) mod pwd;
pub(crate) mod range;
#[allow(unused)]
pub(crate) mod reduce_by;
pub(crate) mod reject;
pub(crate) mod rename;
pub(crate) mod reverse;
pub(crate) mod rm;
pub(crate) mod save;
pub(crate) mod shells;
pub(crate) mod shuffle;
pub(crate) mod size;
pub(crate) mod skip;
pub(crate) mod skip_while;
pub(crate) mod sort_by;
pub(crate) mod split_by;
pub(crate) mod split_column;
pub(crate) mod split_row;
pub(crate) mod sum;
#[allow(unused)]
pub(crate) mod t_sort_by;
pub(crate) mod table;
pub(crate) mod tags;
pub(crate) mod to_bson;
pub(crate) mod to_csv;
pub(crate) mod to_html;
pub(crate) mod to_json;
pub(crate) mod to_md;
pub(crate) mod to_sqlite;
pub(crate) mod to_toml;
pub(crate) mod to_tsv;
pub(crate) mod to_url;
pub(crate) mod to_yaml;
pub(crate) mod trim;
pub(crate) mod uniq;
pub(crate) mod version;
pub(crate) mod what;
pub(crate) mod where_;
pub(crate) mod which_;
pub(crate) mod wrap;

pub(crate) use autoview::Autoview;
pub(crate) use cd::Cd;
pub(crate) use command::{
    per_item_command, whole_stream_command, Command, PerItemCommand, UnevaluatedCallInfo,
    WholeStreamCommand,
};

pub(crate) use append::Append;
pub(crate) use calc::Calc;
pub(crate) use compact::Compact;
pub(crate) use config::Config;
pub(crate) use count::Count;
pub(crate) use cp::Cpy;
pub(crate) use date::Date;
pub(crate) use debug::Debug;
pub(crate) use default::Default;
pub(crate) use du::Du;
pub(crate) use each::Each;
pub(crate) use echo::Echo;
pub(crate) use edit::Edit;
pub(crate) mod kill;
pub(crate) use kill::Kill;
pub(crate) mod clear;
pub(crate) use clear::Clear;
pub(crate) mod touch;
pub(crate) use enter::Enter;
#[allow(unused_imports)]
pub(crate) use evaluate_by::EvaluateBy;
pub(crate) use exit::Exit;
pub(crate) use first::First;
pub(crate) use format::Format;
pub(crate) use from_bson::FromBSON;
pub(crate) use from_csv::FromCSV;
pub(crate) use from_ics::FromIcs;
pub(crate) use from_ini::FromINI;
pub(crate) use from_json::FromJSON;
pub(crate) use from_ods::FromODS;
pub(crate) use from_sqlite::FromDB;
pub(crate) use from_sqlite::FromSQLite;
pub(crate) use from_ssv::FromSSV;
pub(crate) use from_toml::FromTOML;
pub(crate) use from_tsv::FromTSV;
pub(crate) use from_url::FromURL;
pub(crate) use from_vcf::FromVcf;
pub(crate) use from_xlsx::FromXLSX;
pub(crate) use from_xml::FromXML;
pub(crate) use from_yaml::FromYAML;
pub(crate) use from_yaml::FromYML;
pub(crate) use get::Get;
pub(crate) use group_by::GroupBy;
pub(crate) use headers::Headers;
pub(crate) use help::Help;
pub(crate) use histogram::Histogram;
pub(crate) use history::History;
pub(crate) use insert::Insert;
pub(crate) use last::Last;
pub(crate) use lines::Lines;
pub(crate) use ls::Ls;
#[allow(unused_imports)]
pub(crate) use map_max_by::MapMaxBy;
pub(crate) use mkdir::Mkdir;
pub(crate) use mv::Move;
pub(crate) use next::Next;
pub(crate) use nth::Nth;
pub(crate) use open::Open;
pub(crate) use parse::Parse;
pub(crate) use pick::Pick;
pub(crate) use pivot::Pivot;
pub(crate) use prepend::Prepend;
pub(crate) use prev::Previous;
pub(crate) use pwd::Pwd;
pub(crate) use range::Range;
#[allow(unused_imports)]
pub(crate) use reduce_by::ReduceBy;
pub(crate) use reject::Reject;
pub(crate) use rename::Rename;
pub(crate) use reverse::Reverse;
pub(crate) use rm::Remove;
pub(crate) use save::Save;
pub(crate) use shells::Shells;
pub(crate) use shuffle::Shuffle;
pub(crate) use size::Size;
pub(crate) use skip::Skip;
pub(crate) use skip_while::SkipWhile;
pub(crate) use sort_by::SortBy;
pub(crate) use split_by::SplitBy;
pub(crate) use split_column::SplitColumn;
pub(crate) use split_row::SplitRow;
pub(crate) use sum::Sum;
#[allow(unused_imports)]
pub(crate) use t_sort_by::TSortBy;
pub(crate) use table::Table;
pub(crate) use tags::Tags;
pub(crate) use to_bson::ToBSON;
pub(crate) use to_csv::ToCSV;
pub(crate) use to_html::ToHTML;
pub(crate) use to_json::ToJSON;
pub(crate) use to_md::ToMarkdown;
pub(crate) use to_sqlite::ToDB;
pub(crate) use to_sqlite::ToSQLite;
pub(crate) use to_toml::ToTOML;
pub(crate) use to_tsv::ToTSV;
pub(crate) use to_url::ToURL;
pub(crate) use to_yaml::ToYAML;
pub(crate) use touch::Touch;
pub(crate) use trim::Trim;
pub(crate) use uniq::Uniq;
pub(crate) use version::Version;
pub(crate) use what::What;
pub(crate) use where_::Where;
pub(crate) use which_::Which;
pub(crate) use wrap::Wrap;
