# Fix Typo with Search

검색으로 오타를 찾아 고칩니다.

<!-- difficulty: beginner -->

## Before

```text
roses are red
violets are blu
sugar is sweet
```

## After

```text
roses are red
violets are blue
sugar is sweet
```

## Command

```
/blu<ret>cblue<esc>
```

1. `/` 정규식 검색 시작
1. `blu` 오타 입력
1. `<ret>` 검색 확정하고 오타 선택
1. `c` 선택 영역을 변경
1. `blue` 올바른 철자 입력
1. `<esc>` 일반 모드로 복귀
