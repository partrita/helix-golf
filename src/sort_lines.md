# Sort Lines

정렬되지 않은 줄들을 셸 파이프로 정렬합니다.

## Before

```text
banana
apple
cherry
```

## After

```text
apple
banana
cherry
```

## Command

```
%|sort<ret>
```

1. `%` 전체 파일 선택
1. `|` 각 선택 영역을 셸 명령으로 파이프
1. `sort` 정렬 명령 입력
1. `<ret>` 명령 확정하고 정렬된 결과로 교체
