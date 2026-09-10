# Format JSON with jq

한 줄 JSON을 셸 필터로 보기 좋게 펼칩니다.

<!-- difficulty: advanced -->

## Before

```json
{"name": "helix", "stars": 100}
```

## After

```json
{
  "name": "helix",
  "stars": 100
}
```

## Command

```
%|jq .<ret>
```

1. `%` 전체 파일 선택
1. `|` 각 선택 영역을 셸 명령으로 파이프
1. `jq .` 제이슨 포맷 명령 입력
1. `<ret>` 명령 확정하고 포맷 결과로 교체
