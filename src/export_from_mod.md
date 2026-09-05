# Export from Rust Module

각 모듈에 포함된 함수를 re-export(pub use)합니다.

## Before

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;
```

## After

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;

pub use generate_demos::generate_demos;
pub use mdbook_preprocessor::mdbook_preprocessor;
pub use validate::validate;
```

## Command

```
%yp[<space>

<alt-s>gse

cpub use<esc>leypi::
```

1. `%` 3개의 "mod" 구문 모두 선택
1. `yp` 해당 구문들 복제
1. `[<space>` 복제된 3개 구문 위에 빈 줄 추가
1. `<alt-s>gse` 복제된 구문의 각 "mod"에 3개의 선택 영역 생성
1. `cpub use<esc>` 각 "mod"를 "pub use"로 변경
1. `ley` 각 모듈 이름 복사
1. `p` 각 모듈에 모듈명과 동일한 이름의 함수가 있으므로 끝에 모듈 이름 붙여넣기
1. `i::` 사이에 더블 콜론(::) 경로 구분자 삽입
