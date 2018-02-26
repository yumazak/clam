clam
===============
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![](http://meritbadge.herokuapp.com/clam)](https://crates.io/crates/clam)  
EJSライクなRust用のシンプルなテンプレートエンジンです。(ironをちょっとだけ優遇)

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
use clam::template;
use std::collections::HashMap;
fn main() {
    let mut data = HashMap::new();
    data.insert("name", "hoge");
    let html = template::render("view/index.html", data);
    //=>
    //<html>
    //  <p>Hi hoge</p>
    //</html>
}
```

send::htmlでIronResult<Response>を返すのでironで使いやすいです。 
```rust
extern crate clam;
extern crate iron;
use clam::template;
use std::collections::HashMap;
use iron::prelude::*;
fn main() {
    fn top_handler(_: &mut Request) -> IronResult<Response> {
        let mut data = HashMap::new();
        data.insert("name", "hoge");
        send::html("view/index.html", data)
    }
    let _server = Iron::new(top_handler).http("localhost:3000").unwrap();
}
```
