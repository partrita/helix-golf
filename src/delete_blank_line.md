# Delete Blank Line

빈 줄을 yank 없이 삭제합니다.

<!-- difficulty: beginner -->

## Before

```text
apple

banana
```

## After

```text
apple
banana
```

## Command

```
jX<alt-d>
```

1. `j` 아래 줄로 이동
1. `X` 현재 줄 경계까지 선택
1. `<alt-d>` 선택 영역을 yank 없이 삭제
