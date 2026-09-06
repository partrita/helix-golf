# Replace Surrounding Characters

같은 형태로 감싸진 여러 텍스트의 괄호를 한 번에 다른 괄호로 변경합니다.

## Before

```text
(use)
(use)
```

## After

```text
[use]
[use]
```

## Command

```
%suse<ret>mr([
```

1. `%` 문서 전체를 선택
1. `suse<ret>` 선택 영역 안의 모든 텍스트를 정규식으로 찾아 다중 선택
1. `mr([` 선택된 텍스트를 감싸는 괄호를 변경