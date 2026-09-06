# Replace a Selection with the System Clipboard

먼저 코드 일부를 시스템 클립보드에 복사한 뒤, 기존 코드의 일부를 삭제하지 않고 클립보드 내용으로 교체합니다. Helix의 `d`는 삭제한 내용을 yank하기 때문에, 시스템 클립보드를 보존하려면 `d` 대신 시스템 클립보드에 직접 yank하고 `<alt-d>`로 삭제한 뒤 시스템 클립보드에서 붙여넣습니다.

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
/world<ret><space>yng<alt-d><space>p
```

1. `/world<ret>` 문서에서 첫 번째 `world`를 검색하고 선택
1. `<space>y` 선택한 `world`를 시스템 클립보드에 복사
1. `n` 다음 `world` 검색 결과로 이동
1. `g`와 `<alt-d>` 선택 영역을 삭제하되 yank하지 않음
1. `<space>p` 시스템 클립보드의 `world`를 선택 영역 뒤에 붙여넣기

> 이 예제에서는 원래 `world`를 `Rust`로 바꾸기 위해 실제로 복사할 내용이 필요하므로, 복사 단계에서 시스템 클립보드에 이미 `Rust`가 들어 있는 상황을 재현하기보다 문서 안의 `Rust`를 먼저 시스템 클립보드에 복사하는 흐름을 보여주는 것이 더 적절합니다. 따라서 실제 실행 가능한 예제에서는 복사할 위치와 교체할 위치를 구분해야 합니다.

> 핵심은 `d`를 사용하지 않는 것입니다. Helix의 `d`는 삭제한 내용을 yank하므로, 기존 시스템 클립보드 또는 yank 내용을 보존해야 하는 경우 `<alt-d>`(`delete_selection_noyank`)를 사용합니다.
