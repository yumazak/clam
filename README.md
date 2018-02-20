clam
===============
[![](http://meritbadge.herokuapp.com/clam)](https://crates.io/crates/clam)  
EJSライクなRust用のシンプルなテンプレートエンジンです。(ironをちょっとだけ優遇)

## Usage


拡張子clmのファイルを読み込み,<%= %>で囲まれたデータを対象にします。    

**index.clm**
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
    let html = template::render(data,"hoge/index");
    //=> 
    //<html>
    //  <p>Hi hoge</p>
    //</html>
}
```
  
addメソッドを使うとIronResult<Response>を返すのでめっちゃ楽です。  
  
**main.rs**
```rust
fn main() {
    fn top_handler(_: &mut Request) -> IronResult<Response> {
        let mut data = HashMap::new();
        data.insert("name", "hoge");
        template::add(data,"view/index")
    }
    let _server = Iron::new(top_handler).http("localhost:3000").unwrap();
}
```
