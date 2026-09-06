# Replace a Selection with the System Clipboard

시스템 클립보드에 복사한 내용을 다른 선택 영역으로 교체합니다. Helix의 `d`는 삭제한 내용을 yank하므로, 시스템 클립보드의 내용을 보존하면서 교체하려면 `<space>R`을 사용합니다.

## Before

```rs
fn greet() {
    println!("Hello, world!");
}

fn main() {
    println!("Hello, Rust!");
}
```

## After

```rs
fn greet() {
    println!("Hello, Rust!");
}

fn main() {
    println!("Hello, Rust!");
}
```

## Command

```
/Rust<ret><space>ygg/world<ret><space>R
```

1. `/Rust<ret>` `Rust`를 검색하고 선택
1. `<space>y` 선택한 `Rust`를 시스템 클립보드에 복사
1. `gg` 파일의 첫 줄로 이동
1. `/world<ret>` `world`를 검색하고 선택
1. `<space>R` 시스템 클립보드의 내용으로 선택 영역을 교체

> `<space>R`은 시스템 클립보드의 내용으로 현재 선택 영역을 바로 교체하므로, `d`나 `<alt-d>`로 먼저 삭제할 필요가 없습니다.
