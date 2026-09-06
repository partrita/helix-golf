# Replace a Selection with the System Clipboard

먼저 코드 일부를 시스템 클립보드에 복사한 뒤, 기존 코드의 일부를 삭제하지 않고 클립보드 내용으로 교체합니다. Helix의 `d`는 삭제한 내용을 yank하기 때문에, 시스템 클립보드를 보존하려면 `d` 대신 시스템 클립보드에 직접 yank하고 `Alt-d`로 삭제한 뒤 시스템 클립보드에서 붙여넣습니다.

## Before

```rs
fn greet() {
    println!("Hello, world!");
}

fn main() {
    println!("Hello, world!");
}
```

## After

```rs
fn greet() {
    println!("Hello, world!");
}

fn main() {
    println!("Hello, Rust!");
}
```

## Command

```
%miw<space>ygg
%miw<space>pgg
```

1. `miw` 첫 번째 `world`의 단어를 선택
1. `<space>y` 선택한 내용을 시스템 클립보드에 복사
1. `gg` 파일의 첫 줄로 이동
1. `%miw` 첫 번째 `world`의 단어를 다시 선택
1. `<space>p` 시스템 클립보드 내용을 선택 영역 뒤에 붙여넣기

> 이 예제의 핵심은 `d`를 사용하지 않는 것입니다. `d`는 삭제한 내용을 Helix의 yank 레지스터에 저장하므로 이후 yank/paste 동작에 영향을 줄 수 있습니다. 삭제가 필요하다면 `<alt-d>`를 사용하면 삭제하면서 yank하지 않습니다.
