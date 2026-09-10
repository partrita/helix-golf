# Swap Quoted Strings

<!-- difficulty: advanced -->

명명 레지스터로 두 따옴표 내용을 교환합니다.

## Before

```text
"foo" "bar"
```

## After

```text
"bar" "foo"
```

## Command

```
/foo<ret>"ay/bar<ret>"by"aR/foo<ret>"bR
```

1. `/` 정규식 검색 시작
1. `foo` 첫 번째 단어 입력
1. `<ret>` 검색 확정하고 단어 선택
1. `"ay` 선택 영역을 에이 레지스터에 복사
1. `/bar<ret>` 두 번째 단어를 검색해 선택
1. `"by` 선택 영역을 비 레지스터에 복사
1. `"aR` 에이 레지스터 내용으로 교체
1. `/foo<ret>` 첫 번째 단어를 검색해 선택
1. `"bR` 비 레지스터 내용으로 교체
