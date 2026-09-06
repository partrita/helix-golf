# Replace an Identifier

반복해서 등장하는 변수 이름을 한 번에 변경합니다.

## Before

```rs
fn main() {
    let count = 1;
    let next = count + 1;
    println!("{}", count);
}
```

## After

```rs
fn main() {
    let total = 1;
    let next = total + 1;
    println!("{}", total);
}
```

## Command

```
:s/count/total/g<ret>
```

1. `:` 명령 모드로 진입
1. `s/count/total/g` 현재 버퍼에서 `count`를 `total`로 모두 치환
1. `<ret>` 명령 실행
