clam
===============
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![](http://meritbadge.herokuapp.com/clam)](https://crates.io/crates/clam)  
EJSライクなRust用のシンプルなテンプレートエンジンです。

## 使い方

指定したファイルを読み込み,<%= %>で囲まれたデータを対象にします。    

**index.html**
```html
<html>
  <p>Hi <%= name %></p>
</html>
```

**main.rs**
```rust
extern crate clam;
extern crate iron;
use clam::template::TemplateBuilder;
use std::collections::HashMap;
use iron::prelude::*;
fn main() {
    fn top_handler(_: &mut Request) -> IronResult<Response> {
        let mut data = HashMap::new();
        data.insert("name", "hoge");
        let html = TemplateBuilder::new("view/index.html")
            .data(data)
            .build()
            .html();
        html
    }
    let _server = Iron::new(top_handler).http("localhost:3000").unwrap();
}
```
