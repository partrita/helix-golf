# Wrap with Tag

각 줄을 여는 태그와 닫는 태그로 감쌉니다.

<!-- difficulty: advanced -->

## Before

```text
apple
banana
```

## After

```text
<li>apple</li>
<li>banana</li>
```

## Command

```
%<alt-s>I<lt>li><esc>%<alt-s>A<lt>/li><esc>
```

1. `%` 전체 파일 선택
1. `<alt-s>` 줄바꿈 기준으로 여러 선택 영역으로 분할
1. `I<lt>li>` 각 줄 앞에서 삽입 모드로 진입하고 여는 태그 입력
1. `<esc>` 일반 모드로 복귀
1. `%` 전체 파일 다시 선택
1. `<alt-s>` 줄바꿈 기준으로 여러 선택 영역으로 분할
1. `A<lt>/li>` 각 줄 끝에서 삽입 모드로 진입하고 닫는 태그 입력
1. `<esc>` 일반 모드로 복귀
