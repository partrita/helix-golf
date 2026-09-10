# Add Semicolons

<!-- difficulty: beginner -->

각 줄 끝에 세미콜론을 붙입니다.

## Before

```js
const a = 1
const b = 2
const c = 3
```

## After

```js
const a = 1;
const b = 2;
const c = 3;
```

## Command

```
%<alt-s>A;<esc>
```

1. `%` 전체 파일 선택
1. `<alt-s>` 줄바꿈 기준으로 여러 선택 영역으로 분할
1. `A` 각 줄 끝에서 삽입 모드로 진입
1. `;` 세미콜론 입력
1. `<esc>` 일반 모드로 복귀
