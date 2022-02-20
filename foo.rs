#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;
    pub struct Config {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Config {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "server_addr" => _serde::__private::Ok(__Field::__field0),
                            "pg" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"server_addr" => _serde::__private::Ok(__Field::__field0),
                            b"pg" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Config>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Config;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Config")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Config with 2 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            deadpool_postgres::Config,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Config with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(Config {
                            server_addr: __field0,
                            pg: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<deadpool_postgres::Config> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "server_addr",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "pg",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            deadpool_postgres::Config,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("server_addr") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("pg") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Config {
                            server_addr: __field0,
                            pg: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["server_addr", "pg"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Config",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Config>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}
pub mod db {
    use crate::{errors::MyError, models::*};
    use bcrypt::verify;
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;
    pub async fn add_article(client: &Client, article: Article) -> Result<(), MyError> {
        let _stmt = "INSERT INTO articles(name_ref, name, content, description, thumbnail, author_id) VALUES ($1, $2, $3, $4, $5, $6);" ;
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &article.name_ref,
                    &article.name,
                    &article.content,
                    &article.description,
                    &article.thumbnail,
                    &article.author_id,
                ],
            )
            .await?;
        Ok(())
    }
    pub async fn update_article(client: &Client, article: &Article) -> Result<(), MyError> {
        let _stmt = "UPDATE articles SET name=$1, content=$2, description=$3 WHERE name_ref=$4;";
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &article.name,
                    &article.content,
                    &article.description,
                    &article.name_ref,
                ],
            )
            .await
            .unwrap();
        Ok(())
    }
    pub async fn get_article(
        client: &Client,
        article_ref: String,
    ) -> Result<(Article, Author), MyError> {
        let _stmt = "SELECT * FROM articles JOIN authors ON articles.author_id=authors.id WHERE name_ref=$1 LIMIT 1;" ;
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(&stmt, &[&article_ref])
            .await?
            .iter()
            .map(|row| {
                (
                    Article::from_row_ref(row).unwrap(),
                    Author::from_row_ref(row).unwrap(),
                )
            })
            .collect::<Vec<(Article, Author)>>()
            .pop()
            .ok_or(MyError::NotFound)
    }
    pub async fn get_articles(client: &Client) -> Result<Vec<Article>, MyError> {
        let _stmt = "SELECT * FROM articles ORDER BY created_date DESC;";
        let stmt = client.prepare(&_stmt).await.unwrap();
        Ok(client
            .query(&stmt, &[])
            .await?
            .iter()
            .map(|row| Article::from_row_ref(row).unwrap())
            .collect::<Vec<Article>>())
    }
    pub async fn auth_submission(
        client: &Client,
        sbmsn: &ArticleSubmission,
    ) -> Result<SubmissionResult, MyError> {
        let _stmt = "SELECT * FROM authors WHERE nick=$1;";
        let stmt = client.prepare(&_stmt).await.unwrap();
        let author = client
            .query(&stmt, &[&sbmsn.author_name])
            .await?
            .iter()
            .map(|row| Author::from_row_ref(row).unwrap())
            .collect::<Vec<Author>>()
            .pop();
        if let None = author {
            return Ok(SubmissionResult::Rejected(
                "Nickname and author do not match".into(),
            ));
        }
        let author = author.unwrap();
        let valid = verify(sbmsn.password.clone(), &author.password).unwrap();
        if valid {
            let article = sbmsn.to_article(author.id);
            return Ok(SubmissionResult::Accepted(article));
        }
        Ok(SubmissionResult::Rejected(
            "Nickname and password do not match".into(),
        ))
    }
    pub async fn article_exists(
        client: &Client,
        sbmsn: &ArticleSubmission,
    ) -> Result<bool, MyError> {
        let _stmt = "SELECT id FROM articles WHERE name_ref=$1;";
        let stmt = client.prepare(&_stmt).await.unwrap();
        let articles = client
            .query(&stmt, &[&sbmsn.name_ref])
            .await?
            .iter()
            .map(|row| row.get("id"))
            .collect::<Vec<i32>>();
        match articles.len() {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
}
pub mod errors {
    use crate::files::build_html;
    use crate::{Page, Meta};
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;
    pub enum MyError {
        NotFound,
        NotAuthorized,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl ::core::fmt::Display for MyError {
        #[allow(unused_variables)]
        #[inline]
        fn fmt(
            &self,
            _derive_more_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            match self {
                MyError::NotFound => _derive_more_display_formatter.write_str("NotFound"),
                MyError::NotAuthorized => _derive_more_display_formatter.write_str("NotAuthorized"),
                MyError::PGError(_0) => {
                    ::core::fmt::Display::fmt(_0, _derive_more_display_formatter)
                }
                MyError::PGMError(_0) => {
                    ::core::fmt::Display::fmt(_0, _derive_more_display_formatter)
                }
                MyError::PoolError(_0) => {
                    ::core::fmt::Display::fmt(_0, _derive_more_display_formatter)
                }
                _ => Ok(()),
            }
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(PoolError)> for MyError {
        #[inline]
        fn from(original: (PoolError)) -> MyError {
            MyError::PoolError(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(PGError)> for MyError {
        #[inline]
        fn from(original: (PGError)) -> MyError {
            MyError::PGError(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(PGMError)> for MyError {
        #[inline]
        fn from(original: (PGMError)) -> MyError {
            MyError::PGMError(original)
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MyError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&MyError::NotFound,) => ::core::fmt::Formatter::write_str(f, "NotFound"),
                (&MyError::NotAuthorized,) => ::core::fmt::Formatter::write_str(f, "NotAuthorized"),
                (&MyError::PGError(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "PGError");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&MyError::PGMError(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "PGMError");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&MyError::PoolError(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "PoolError");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl std::error::Error for MyError {}
    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => {
                    let md = "# 404 - Obsah nenalezen\n---\n\nBohu\u{17e}el v\u{e1}mi hledan\u{fd} obsah neexistuje.\n\t\n<div align=\"center\">\n    <img src=\"https://cdn.7tv.app/emote/60b05d483cadd71dffc67135/4x\\\">\n</div>" ;
                    let meta = Meta::from([("TITLE".into(), "Nenalezeno - smolik.xyz".into())]);
                    let html = build_html(md.into(), meta, Page::Error);
                    HttpResponse::Ok().content_type("text/html").body(html)
                }
                MyError::NotAuthorized => {
                    let md = "# 401 - P\u{159}\u{ed}stup nepovolen\n---\n\nK vykon\u{e1}n\u{ed} t\u{e9}to akce nejste opr\u{e1}vn\u{11b}ni.\n\t\n<div align=\"center\">\n    <img src=\"https://cdn.7tv.app/emote/60ae69d29627f9aff4433b74/4x\">\n</div>" ;
                    let meta =
                        Meta::from([("TITLE".into(), "Nejste oprávněni - smolik.xyz".into())]);
                    let html = build_html(md.into(), meta, Page::Error);
                    HttpResponse::Ok().content_type("text/html").body(html)
                }
                _ => {
                    let md = "# 500 - Vnit\u{159}n\u{ed} chyba serveru\n---\n\nBohu\u{17e}el p\u{159}i zpracov\u{e1}n\u{ed} va\u{161}eho po\u{17e}adavku nastala chyba, omlouv\u{e1}m se.\n\n<div align=\"center\">\n    <img src=\"https://cdn.7tv.app/emote/6140b4f3962a6090486467f1/4x\">\n</div>" ;
                    let html = build_html(md.into(), Meta::new(), Page::Error);
                    HttpResponse::Ok().content_type("text/html").body(html)
                }
            }
        }
    }
}
pub mod files {
    use crate::{Page, Meta};
    use pulldown_cmark::{html, Options, Parser};
    pub fn build_html(md: String, meta: Meta, page: Page) -> String {
        let mut html_template : String = match page { Page :: Article => "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <link rel=\"stylesheet\" href=\"/main.min.css\">\n\t<script src=\"/highlight.min.js\"></script>\n\t<link rel=\"stylesheet\" href=\"/tokyo-night-dark.min.css\">\n    <title> {{ TITLE }} </title>\n</head>\n<body>\n    <header>\n        <a href=\"/\">\n            <div>Dom\u{16f}</div>\n        </a>\n        <a href=\"/about\">\n            <div>O mn\u{11b}</div>\n        </a>\n        <a href=\"/\">\n            <div>Kontakty</div>\n        </a>\n    </header>\n\n    <div class=\"container\">\n        {{ CONTENT }}\n\n        <footer>\n            <table>\n                <tr>\n                    <td>Autor:</td>\n                    <td>{{ AUTHOR }} <img class=\"author-logo\" src=\"/avatars/{{ AVATAR }}.webp\"/></td>\n                </tr>\n                <tr>\n                    <td>Datum:</td>\n                    <td>{{ CREATED }}</td>\n                </tr>\n                <tr>\n                    <td>Posledn\u{ed} \u{fa}prava:</td>\n                    <td>{{ LAST_EDIT }}</td>\n                </tr>\n            </table>\n        </footer>\n    </div>\n\n\t<script>hljs.highlightAll();</script>\n    <script src=\"/main.min.js\"></script>\n</body>\n</html>" . into () , Page :: EditingArticle => "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <link rel=\"stylesheet\" href=\"/main.min.css\">\n    <title> {{ TITLE }} </title>\n</head>\n<body>\n    <header>\n        <a href=\"/\">\n            <div>Dom\u{16f}</div>\n        </a>\n        <a href=\"/about\">\n            <div>O mn\u{11b}</div>\n        </a>\n        <a href=\"/\">\n            <div>Kontakty</div>\n        </a>\n    </header>\n\n\t<div class=\"container center-content form-container\">\n\t\t<div class=\"form-group field\">\n\t\t\t<input\n\t\t\t\ttype=\"text\"\n\t\t\t\tname=\"name\"\n\t\t\t\tclass=\"form-field\"\n\t\t\t\tvalue=\"{{ NAME }}\"\n\t\t\t\tplaceholder=\"N\u{e1}zev\"\n\t\t\t\tid=\"articleName\">\n\t\t\t<label for=\"name\" class=\"form-label\">N\u{e1}zev</label>\n\t\t</div>\n\t\t<div class=\"form-group field\">\n\t\t\t<input\n\t\t\t\ttype=\"text\"\n\t\t\t\tname=\"description\"\n\t\t\t\tclass=\"form-field\"\n\t\t\t\tvalue=\"{{ DESCRIPTION }}\"\n\t\t\t\tplaceholder=\"Popis\" \n\t\t\t\tid=\"articleDescription\">\n\t\t\t<label for=\"description\" class=\"form-label\">Popis</label>\n\t\t</div>\n\t\t<div class=\"form-group field\">\n\t\t\t<input\n\t\t\t\ttype=\"text\"\n\t\t\t\tname=\"author\"\n\t\t\t\tclass=\"form-field\"\n\t\t\t\tvalue=\"{{ AUTHOR }}\"\n\t\t\t\tplaceholder=\"Autor\" \n\t\t\t\tid=\"authorName\">\n\t\t\t<label for=\"author\" class=\"form-label\">Autor</label>\n\t\t</div>\n\t\t<div class=\"form-group field\">\n\t\t\t<input\n\t\t\t\ttype=\"password\"\n\t\t\t\tname=\"password\"\n\t\t\t\tclass=\"form-field\"\n\t\t\t\tvalue=\"\"\n\t\t\t\tplaceholder=\"Heslo\" \n\t\t\t\tid=\"_authorPassword\">\n\t\t\t<label for=\"password\" class=\"form-label\">Heslo</label>\n\t\t</div>\n\t\t<br>\n\t\t<input\n\t\t\ttype=\"file\"\n\t\t\tvalue=\"Nahr\u{e1}t miniaturu\"\n\t\t\tid=\"thumbnail\">\n\t</div>\n\n    <div class=\"container editing-article_wrapper\">\n\t\t<textarea class=\"editing-article\" id=\"articleContent\">{{ CONTENT }}</textarea>\n    </div>\n\n\t<div class=\"container response-wrapper\">\n\t\t<button id=\"submitBtn\">Ulo\u{17e}it</button>\n\t\t<span id=\"responseText\"><img src=\"/loading.svg\" id=\"loadingIcon\"></span>\n\t</div>\n\n    <script src=\"/article_edit.min.js\"></script>\n</body>\n</html>\n" . into () , _ => "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <link rel=\"stylesheet\" href=\"/main.min.css\">\n    <title> {{ TITLE }} </title>\n</head>\n<body>\n    <header>\n        <a href=\"/\">\n            <div>Dom\u{16f}</div>\n        </a>\n        <a href=\"/about\">\n            <div>O mn\u{11b}</div>\n        </a>\n        <a href=\"/\">\n            <div>Kontakty</div>\n        </a>\n    </header>\n\n    <div class=\"container\">\n        {{ CONTENT }}\n    </div>\n</body>\n</html>" . into () , } ;
        html_template = match page {
            Page::EditingArticle => html_template.replace("{{ CONTENT }}", &md),
            _ => {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                options.insert(Options::ENABLE_TABLES);
                let parser = Parser::new_ext(&md, options);
                let mut html_output: String = String::with_capacity(md.len() * 3 / 2);
                html::push_html(&mut html_output, parser);
                html_template.replace("{{ CONTENT }}", &html_output)
            }
        };
        for (key, val) in meta.into_iter() {
            html_template = html_template.replace(
                &{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["{{ ", " }}"],
                        &[::core::fmt::ArgumentV1::new_display(&key)],
                    ));
                    res
                },
                &val,
            )
        }
        html_template
    }
}
pub mod handlers {
    use crate::db;
    use crate::errors::MyError;
    use crate::files::{build_html};
    use crate::{
        Page, Meta,
        models::{ArticleSubmission, SubmissionResult, SubmissionResponseJson},
    };
    use actix_web::{web, Error, HttpResponse, Responder, get, put, post};
    use deadpool_postgres::{Client, Pool};
    #[allow(non_camel_case_types, missing_docs)]
    pub struct get_home;
    impl ::actix_web::dev::HttpServiceFactory for get_home {
        fn register(self, __config: &mut actix_web::dev::AppService) {
            pub async fn get_home(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
                let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
                let articles = db::get_articles(&client).await?;
                let mut html : String = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <link rel=\"stylesheet\" href=\"/main.min.css\">\n    <title> {{ TITLE }} </title>\n</head>\n<body>\n    <header>\n        <a href=\"/\">\n            <div>Dom\u{16f}</div>\n        </a>\n        <a href=\"/about\">\n            <div>O mn\u{11b}</div>\n        </a>\n        <a href=\"/\">\n            <div>Kontakty</div>\n        </a>\n    </header>\n\n    <div class=\"container\">\n        {{ CONTENT }}\n    </div>\n</body>\n</html>" . into () ;
                let mut links = String::new();
                for a in articles.into_iter() {
                    links += &{
                        let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["\n\t\t\t<a href=\"/articles/" , "\">\n\t\t\t\t<div class=\"article\">\n\t\t\t\t\t<h2 class=\"article-name\">" , "</h2>\n\t\t\t\t\t<hr>\n\t\t\t\t\t<div class=\"article-body_wrapper\">\n\t\t\t\t\t\t<img class=\"article-img\" src=\"/thumbnails/" , ".webp\">\n\t\t\t\t\t\t<div class=\"article-desc\">" , "</div>\n\t\t\t\t\t</div>\n\t\t\t\t</div>\n\t\t\t</a>\n\t\t"] , & [:: core :: fmt :: ArgumentV1 :: new_display (& a . name_ref) , :: core :: fmt :: ArgumentV1 :: new_display (& a . name) , :: core :: fmt :: ArgumentV1 :: new_display (& a . thumbnail) , :: core :: fmt :: ArgumentV1 :: new_display (& a . description)])) ;
                        res
                    };
                }
                html = html
                    .replace("{{ CONTENT }}", &links)
                    .replace("{{ TITLE }}", "smolik.xyz");
                Ok(HttpResponse::Ok().content_type("text/html").body(html))
            }
            let __resource = ::actix_web::Resource::new("/")
                .name("get_home")
                .guard(::actix_web::guard::Get())
                .to(get_home);
            ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
        }
    }
    #[allow(non_camel_case_types, missing_docs)]
    pub struct get_about;
    impl ::actix_web::dev::HttpServiceFactory for get_about {
        fn register(self, __config: &mut actix_web::dev::AppService) {
            pub async fn get_about() -> impl Responder {
                let md : String = "# O mn\u{11b}\n---\n\nJmenuji se Marek Smol\u{ed}k a jsem nad\u{161}enec do IT, programov\u{e1}n\u{ed} a matematiky.\n\nMoment\u{e1}ln\u{11b} studuji Informa\u{10d}n\u{ed} technologie na st\u{159}edn\u{ed} \u{161}kole.\n\nN\u{11b}kter\u{e9} z m\u{fd}ch projekt\u{16f} a prac\u{ed} mohou zahrnovat:\n- [Programov\u{e1}n\u{ed} v jazyce Rust](http://cdn.smolik.xyz/matura.pdf) - maturitn\u{ed} pr\u{e1}ce popisuj\u{ed}c\u{ed} programovac\u{ed} jazyk Rust (2022)." . into () ;
                let meta = Meta::from([("{{ TITLE }}".into(), "O mně - smolik.xyz".into())]);
                let html = build_html(md, meta, Page::About);
                HttpResponse::Ok().content_type("text/html").body(html)
            }
            let __resource = ::actix_web::Resource::new("/about")
                .name("get_about")
                .guard(::actix_web::guard::Get())
                .to(get_about);
            ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
        }
    }
    #[allow(non_camel_case_types, missing_docs)]
    pub struct get_article;
    impl ::actix_web::dev::HttpServiceFactory for get_article {
        fn register(self, __config: &mut actix_web::dev::AppService) {
            pub async fn get_article(
                db_pool: web::Data<Pool>,
                article_ref: web::Path<String>,
            ) -> Result<HttpResponse, Error> {
                let article_ref = article_ref.into_inner();
                let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
                let (article, author) = db::get_article(&client, article_ref.clone()).await?;
                let meta = Meta::from([
                    ("TITLE".into(), {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", " - smolik.xyz"],
                            &[::core::fmt::ArgumentV1::new_display(&article.name)],
                        ));
                        res
                    }),
                    ("CREATED".into(), {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&article.created_date)],
                        ));
                        res
                    }),
                    ("LAST_EDIT".into(), {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&article.edit_date)],
                        ));
                        res
                    }),
                    ("AUTHOR".into(), author.nick.into()),
                    ("AVATAR".into(), {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&author.avatar)],
                        ));
                        res
                    }),
                ]);
                let html = build_html(article.content, meta, Page::Article);
                Ok(HttpResponse::Ok().content_type("text/html").body(html))
            }
            let __resource = ::actix_web::Resource::new("/articles/{article_ref}")
                .name("get_article")
                .guard(::actix_web::guard::Get())
                .to(get_article);
            ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
        }
    }
    pub async fn not_found() -> impl Responder {
        let md = "# 404 - Obsah nenalezen\n---\n\nBohu\u{17e}el v\u{e1}mi hledan\u{fd} obsah neexistuje.\n\t\n<div align=\"center\">\n    <img src=\"https://cdn.7tv.app/emote/60b05d483cadd71dffc67135/4x\\\">\n</div>" ;
        let html = build_html(md.into(), Meta::new(), Page::Error);
        HttpResponse::Ok().content_type("text/html").body(html)
    }
    #[allow(non_camel_case_types, missing_docs)]
    pub struct edit_article;
    impl ::actix_web::dev::HttpServiceFactory for edit_article {
        fn register(self, __config: &mut actix_web::dev::AppService) {
            pub async fn edit_article(
                db_pool: web::Data<Pool>,
                article_ref: web::Path<String>,
            ) -> Result<HttpResponse, Error> {
                let article_ref = article_ref.into_inner();
                let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
                let (article, author) = db::get_article(&client, article_ref).await?;
                let meta = Meta::from([
                    ("TITLE".into(), {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["\u{da}prava \u{201e}", "\u{201c} - smolik.xyz"],
                            &[::core::fmt::ArgumentV1::new_display(&article.name)],
                        ));
                        res
                    }),
                    ("NAME".into(), article.name),
                    ("DESCRIPTION".into(), article.description),
                    ("AUTHOR".into(), author.nick),
                ]);
                let html = build_html(article.content, meta, Page::EditingArticle);
                Ok(HttpResponse::Ok().content_type("text/html").body(html))
            }
            let __resource = ::actix_web::Resource::new("/edit/{article_ref}")
                .name("edit_article")
                .guard(::actix_web::guard::Get())
                .to(edit_article);
            ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
        }
    }
    #[allow(non_camel_case_types, missing_docs)]
    pub struct add_article;
    impl ::actix_web::dev::HttpServiceFactory for add_article {
        fn register(self, __config: &mut actix_web::dev::AppService) {
            pub async fn add_article() -> Result<HttpResponse, Error> {
                let meta = Meta::from([
                    ("TITLE".into(), "Tvorba nového článku - smolik.xyz".into()),
                    ("NAME".into(), "".into()),
                    ("DESCRIPTION".into(), "".into()),
                    ("AUTHOR".into(), "".into()),
                ]);
                let html = build_html("".into(), meta, Page::EditingArticle);
                Ok(HttpResponse::Ok().content_type("text/html").body(html))
            }
            let __resource = ::actix_web::Resource::new("/new/{article_ref}")
                .name("add_article")
                .guard(::actix_web::guard::Get())
                .to(add_article);
            ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
        }
    }
    #[allow(non_camel_case_types, missing_docs)]
    pub struct put_article;
    impl ::actix_web::dev::HttpServiceFactory for put_article {
        fn register(self, __config: &mut actix_web::dev::AppService) {
            pub async fn put_article(
                db_pool: web::Data<Pool>,
                article: web::Json<ArticleSubmission>,
            ) -> Result<HttpResponse, Error> {
                let submission = article.into_inner();
                let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
                let auth = crate::db::auth_submission(&client, &submission).await?;
                if let SubmissionResult::Rejected(_) = auth {
                    return Ok(HttpResponse::Ok().json(SubmissionResponseJson::unauthorized()));
                }
                let article = match auth {
                    SubmissionResult::Accepted(a) => a,
                    _ => ::core::panicking::panic("internal error: entered unreachable code"),
                };
                let exists = db::article_exists(&client, &submission).await?;
                if !exists {
                    return Ok(HttpResponse::Ok().json(SubmissionResponseJson::not_found()));
                }
                db::update_article(&client, &article).await?;
                Ok(HttpResponse::Ok().json(SubmissionResponseJson::edited_succ()))
            }
            let __resource = ::actix_web::Resource::new("/api/article")
                .name("put_article")
                .guard(::actix_web::guard::Put())
                .to(put_article);
            ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
        }
    }
    #[allow(non_camel_case_types, missing_docs)]
    pub struct post_article;
    impl ::actix_web::dev::HttpServiceFactory for post_article {
        fn register(self, __config: &mut actix_web::dev::AppService) {
            pub async fn post_article(
                article: web::Json<ArticleSubmission>,
                db_pool: web::Data<Pool>,
            ) -> Result<HttpResponse, Error> {
                let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
                let submission: ArticleSubmission = article.into_inner();
                let auth = db::auth_submission(&client, &submission).await?;
                match auth {
                    SubmissionResult::Accepted(article) => {
                        db::add_article(&client, article).await?;
                        Ok(HttpResponse::Ok().json(SubmissionResponseJson::created_succ()))
                    }
                    SubmissionResult::Rejected(_) => {
                        Ok(HttpResponse::Ok().json(SubmissionResponseJson::unauthorized()))
                    }
                }
            }
            let __resource = ::actix_web::Resource::new("/api/article")
                .name("post_article")
                .guard(::actix_web::guard::Post())
                .to(post_article);
            ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
        }
    }
}
pub mod models {
    use chrono::{NaiveDate, Datelike};
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;
    #[pg_mapper(table = "authors")]
    pub struct Author {
        pub id: i32,
        pub avatar: String,
        pub nick: String,
        pub bio: String,
        pub registered_date: chrono::NaiveDate,
        pub password: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Author {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Author {
                    id: ref __self_0_0,
                    avatar: ref __self_0_1,
                    nick: ref __self_0_2,
                    bio: ref __self_0_3,
                    registered_date: ref __self_0_4,
                    password: ref __self_0_5,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Author");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "avatar",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "nick",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "bio",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "registered_date",
                        &&(*__self_0_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "password",
                        &&(*__self_0_5),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Author {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "avatar" => _serde::__private::Ok(__Field::__field1),
                            "nick" => _serde::__private::Ok(__Field::__field2),
                            "bio" => _serde::__private::Ok(__Field::__field3),
                            "registered_date" => _serde::__private::Ok(__Field::__field4),
                            "password" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"avatar" => _serde::__private::Ok(__Field::__field1),
                            b"nick" => _serde::__private::Ok(__Field::__field2),
                            b"bio" => _serde::__private::Ok(__Field::__field3),
                            b"registered_date" => _serde::__private::Ok(__Field::__field4),
                            b"password" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Author>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Author;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Author")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Author with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Author with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Author with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Author with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            chrono::NaiveDate,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct Author with 6 elements",
                                ));
                            }
                        };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Author with 6 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Author {
                            id: __field0,
                            avatar: __field1,
                            nick: __field2,
                            bio: __field3,
                            registered_date: __field4,
                            password: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<chrono::NaiveDate> =
                            _serde::__private::None;
                        let mut __field5: _serde::__private::Option<String> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "avatar",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "nick",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "bio",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "registered_date",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<chrono::NaiveDate>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "password",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("avatar") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("nick") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("bio") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("registered_date") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("password") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Author {
                            id: __field0,
                            avatar: __field1,
                            nick: __field2,
                            bio: __field3,
                            registered_date: __field4,
                            password: __field5,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["id", "avatar", "nick", "bio", "registered_date", "password"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Author",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Author>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl tokio_pg_mapper::FromTokioPostgresRow for Author {
        fn from_row(
            row: tokio_postgres::row::Row,
        ) -> ::std::result::Result<Self, tokio_pg_mapper::Error> {
            Ok(Self {
                id: row.try_get::<&str, i32>("id")?,
                avatar: row.try_get::<&str, String>("avatar")?,
                nick: row.try_get::<&str, String>("nick")?,
                bio: row.try_get::<&str, String>("bio")?,
                registered_date: row.try_get::<&str, chrono::NaiveDate>("registered_date")?,
                password: row.try_get::<&str, String>("password")?,
            })
        }
        fn from_row_ref(
            row: &tokio_postgres::row::Row,
        ) -> ::std::result::Result<Self, tokio_pg_mapper::Error> {
            Ok(Self {
                id: row.try_get::<&str, i32>(&"id")?,
                avatar: row.try_get::<&str, String>(&"avatar")?,
                nick: row.try_get::<&str, String>(&"nick")?,
                bio: row.try_get::<&str, String>(&"bio")?,
                registered_date: row.try_get::<&str, chrono::NaiveDate>(&"registered_date")?,
                password: row.try_get::<&str, String>(&"password")?,
            })
        }
        fn sql_table() -> String {
            "authors".to_string()
        }
        fn sql_table_fields() -> String {
            " authors.id ,  authors.avatar ,  authors.nick ,  authors.bio ,  authors.registered_date ,  authors.password " . to_string ()
        }
        fn sql_fields() -> String {
            " id ,  avatar ,  nick ,  bio ,  registered_date ,  password ".to_string()
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Author {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Author",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "avatar",
                    &self.avatar,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "nick",
                    &self.nick,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "bio",
                    &self.bio,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "registered_date",
                    &self.registered_date,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "password",
                    &self.password,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[pg_mapper(table = "authors")]
    pub struct Article {
        pub id: i32,
        pub name_ref: String,
        pub name: String,
        pub content: String,
        pub description: String,
        pub thumbnail: String,
        pub edit_date: chrono::NaiveDate,
        pub created_date: chrono::NaiveDate,
        pub author_id: i32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Article {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Article {
                    id: ref __self_0_0,
                    name_ref: ref __self_0_1,
                    name: ref __self_0_2,
                    content: ref __self_0_3,
                    description: ref __self_0_4,
                    thumbnail: ref __self_0_5,
                    edit_date: ref __self_0_6,
                    created_date: ref __self_0_7,
                    author_id: ref __self_0_8,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Article");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name_ref",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "content",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "description",
                        &&(*__self_0_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "thumbnail",
                        &&(*__self_0_5),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "edit_date",
                        &&(*__self_0_6),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "created_date",
                        &&(*__self_0_7),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "author_id",
                        &&(*__self_0_8),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Article {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "name_ref" => _serde::__private::Ok(__Field::__field1),
                            "name" => _serde::__private::Ok(__Field::__field2),
                            "content" => _serde::__private::Ok(__Field::__field3),
                            "description" => _serde::__private::Ok(__Field::__field4),
                            "thumbnail" => _serde::__private::Ok(__Field::__field5),
                            "edit_date" => _serde::__private::Ok(__Field::__field6),
                            "created_date" => _serde::__private::Ok(__Field::__field7),
                            "author_id" => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"name_ref" => _serde::__private::Ok(__Field::__field1),
                            b"name" => _serde::__private::Ok(__Field::__field2),
                            b"content" => _serde::__private::Ok(__Field::__field3),
                            b"description" => _serde::__private::Ok(__Field::__field4),
                            b"thumbnail" => _serde::__private::Ok(__Field::__field5),
                            b"edit_date" => _serde::__private::Ok(__Field::__field6),
                            b"created_date" => _serde::__private::Ok(__Field::__field7),
                            b"author_id" => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Article>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Article;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Article")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Article with 9 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Article with 9 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Article with 9 elements",
                                        ),
                                    );
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Article with 9 elements",
                                        ),
                                    );
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Article with 9 elements",
                                        ),
                                    );
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Article with 9 elements",
                                        ),
                                    );
                                }
                            };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            chrono::NaiveDate,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct Article with 9 elements",
                                ));
                            }
                        };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            chrono::NaiveDate,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct Article with 9 elements",
                                ));
                            }
                        };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Article with 9 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Article {
                            id: __field0,
                            name_ref: __field1,
                            name: __field2,
                            content: __field3,
                            description: __field4,
                            thumbnail: __field5,
                            edit_date: __field6,
                            created_date: __field7,
                            author_id: __field8,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field5: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field6: _serde::__private::Option<chrono::NaiveDate> =
                            _serde::__private::None;
                        let mut __field7: _serde::__private::Option<chrono::NaiveDate> =
                            _serde::__private::None;
                        let mut __field8: _serde::__private::Option<i32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name_ref",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "content",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "thumbnail",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "edit_date",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<chrono::NaiveDate>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "created_date",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<chrono::NaiveDate>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "author_id",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name_ref") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("content") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("description") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("thumbnail") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("edit_date") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("created_date") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("author_id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Article {
                            id: __field0,
                            name_ref: __field1,
                            name: __field2,
                            content: __field3,
                            description: __field4,
                            thumbnail: __field5,
                            edit_date: __field6,
                            created_date: __field7,
                            author_id: __field8,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "name_ref",
                    "name",
                    "content",
                    "description",
                    "thumbnail",
                    "edit_date",
                    "created_date",
                    "author_id",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Article",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Article>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl tokio_pg_mapper::FromTokioPostgresRow for Article {
        fn from_row(
            row: tokio_postgres::row::Row,
        ) -> ::std::result::Result<Self, tokio_pg_mapper::Error> {
            Ok(Self {
                id: row.try_get::<&str, i32>("id")?,
                name_ref: row.try_get::<&str, String>("name_ref")?,
                name: row.try_get::<&str, String>("name")?,
                content: row.try_get::<&str, String>("content")?,
                description: row.try_get::<&str, String>("description")?,
                thumbnail: row.try_get::<&str, String>("thumbnail")?,
                edit_date: row.try_get::<&str, chrono::NaiveDate>("edit_date")?,
                created_date: row.try_get::<&str, chrono::NaiveDate>("created_date")?,
                author_id: row.try_get::<&str, i32>("author_id")?,
            })
        }
        fn from_row_ref(
            row: &tokio_postgres::row::Row,
        ) -> ::std::result::Result<Self, tokio_pg_mapper::Error> {
            Ok(Self {
                id: row.try_get::<&str, i32>(&"id")?,
                name_ref: row.try_get::<&str, String>(&"name_ref")?,
                name: row.try_get::<&str, String>(&"name")?,
                content: row.try_get::<&str, String>(&"content")?,
                description: row.try_get::<&str, String>(&"description")?,
                thumbnail: row.try_get::<&str, String>(&"thumbnail")?,
                edit_date: row.try_get::<&str, chrono::NaiveDate>(&"edit_date")?,
                created_date: row.try_get::<&str, chrono::NaiveDate>(&"created_date")?,
                author_id: row.try_get::<&str, i32>(&"author_id")?,
            })
        }
        fn sql_table() -> String {
            "authors".to_string()
        }
        fn sql_table_fields() -> String {
            " authors.id ,  authors.name_ref ,  authors.name ,  authors.content ,  authors.description ,  authors.thumbnail ,  authors.edit_date ,  authors.created_date ,  authors.author_id " . to_string ()
        }
        fn sql_fields() -> String {
            " id ,  name_ref ,  name ,  content ,  description ,  thumbnail ,  edit_date ,  created_date ,  author_id " . to_string ()
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Article {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Article",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name_ref",
                    &self.name_ref,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "content",
                    &self.content,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "thumbnail",
                    &self.thumbnail,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "edit_date",
                    &self.edit_date,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "created_date",
                    &self.created_date,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "author_id",
                    &self.author_id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct ArticleSubmission {
        pub name_ref: String,
        pub name: String,
        pub content: String,
        pub author_name: String,
        pub password: String,
        pub description: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ArticleSubmission {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name_ref" => _serde::__private::Ok(__Field::__field0),
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "content" => _serde::__private::Ok(__Field::__field2),
                            "author_name" => _serde::__private::Ok(__Field::__field3),
                            "password" => _serde::__private::Ok(__Field::__field4),
                            "description" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name_ref" => _serde::__private::Ok(__Field::__field0),
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"content" => _serde::__private::Ok(__Field::__field2),
                            b"author_name" => _serde::__private::Ok(__Field::__field3),
                            b"password" => _serde::__private::Ok(__Field::__field4),
                            b"description" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ArticleSubmission>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ArticleSubmission;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ArticleSubmission",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ArticleSubmission with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ArticleSubmission with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ArticleSubmission with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct ArticleSubmission with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct ArticleSubmission with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct ArticleSubmission with 6 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(ArticleSubmission {
                            name_ref: __field0,
                            name: __field1,
                            content: __field2,
                            author_name: __field3,
                            password: __field4,
                            description: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field5: _serde::__private::Option<String> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name_ref",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "content",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "author_name",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "password",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name_ref") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("content") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("author_name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("password") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("description") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(ArticleSubmission {
                            name_ref: __field0,
                            name: __field1,
                            content: __field2,
                            author_name: __field3,
                            password: __field4,
                            description: __field5,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "name_ref",
                    "name",
                    "content",
                    "author_name",
                    "password",
                    "description",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ArticleSubmission",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ArticleSubmission>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ArticleSubmission {
        pub fn to_article(&self, author_id: i32) -> Article {
            let now = chrono::Utc::now();
            let curr_date = NaiveDate::from_ymd(now.year(), now.month(), now.day());
            Article {
                id: 0,
                author_id: author_id,
                content: self.content.clone(),
                created_date: curr_date,
                edit_date: curr_date,
                description: self.description.clone(),
                name: self.name.clone(),
                name_ref: self.name_ref.clone(),
                thumbnail: "_default".into(),
            }
        }
    }
    pub enum SubmissionResult {
        Accepted(Article),
        Rejected(String),
    }
    pub struct SubmissionResponseJson {
        pub status: u16,
        pub message: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SubmissionResponseJson {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "SubmissionResponseJson",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "status",
                    &self.status,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "message",
                    &self.message,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl SubmissionResponseJson {
        pub fn new(message: String, status: u16) -> Self {
            Self {
                message: message,
                status: status,
            }
        }
        pub fn not_found() -> Self {
            Self::new("Článek neexistuje".into(), 404)
        }
        pub fn unauthorized() -> Self {
            Self::new("Nejste obeprávněni".into(), 401)
        }
        pub fn edited_succ() -> Self {
            Self::new("Článek úspěšně upraven".into(), 201)
        }
        pub fn created_succ() -> Self {
            Self::new("Článek úspěšně vytvořen".into(), 200)
        }
    }
}
use rusty_blog::{Page, Meta};
use handlers::{*};
use actix_files::Files;
use actix_web::{web, App, HttpServer, middleware};
use dotenv::dotenv;
use tokio_postgres::NoTls;
fn main() -> std::io::Result<()> {
    <::actix_web::rt::System>::new().block_on(async move {
        {
            dotenv().ok();
            let config = config::Config::from_env().unwrap();
            let pool = config.pg.create_pool(None, NoTls).unwrap();
            let server = HttpServer::new(move || {
                App::new()
                    .wrap(middleware::Logger::default())
                    .wrap(middleware::Compress::default())
                    .app_data(web::Data::new(pool.clone()))
                    .service(get_home)
                    .service(get_about)
                    .service(get_article)
                    .service(edit_article)
                    .service(add_article)
                    .service(put_article)
                    .service(post_article)
                    .service(Files::new("/", "./static/"))
                    .default_service(web::to(|| not_found()))
            })
            .bind(config.server_addr.clone())?
            .run();
            server.await
        }
    })
}
