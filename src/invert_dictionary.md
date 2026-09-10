# Invert Dictionary

<!-- difficulty: advanced -->

딕셔너리의 키-값 쌍을 반전시킵니다.

## Before

```gdscript
var color_to_points = {
    "red" = 0,
    "orange" = 5,
    "yellow" = 10,
    "green" = 15,
    "blue" = 20,
    "purple" = 30,
    "black" = 50,
}
```

## After

```gdscript
var points_to_color = {
    0 = "red",
    5 = "orange",
    10 = "yellow",
    15 = "green",
    20 = "blue",
    30 = "purple",
    50 = "black",
}
```

## Command

```
webS_to_<enter><alt-(>

Qxt,S=<enter>_<alt-(>Q6q
```

1. `web` 공백을 제외한 두 번째 단어 선택
1. `S` 정규식 일치 기준으로 선택 영역 분할. `_to_`를 정규식으로 입력하고 `<enter>`를 눌러 두 부분으로 분할
1. `<alt-(>` 선택 영역 내용의 순서 회전
1. `Q` 첫 번째 입력으로 매크로 녹화 시작
1. `x` 전체 줄 선택
1. `t,` 다음 쉼표 바로 앞까지 선택
1. `S` 정규식 일치 기준으로 전체 줄 분할. `=`를 정규식으로 입력하고 `<enter>`를 눌러 두 부분으로 분할
1. `_` 모든 선택 영역의 후행 공백 제거
1. `<alt-(>` 선택 영역 내용의 순서 회전
1. `Q` 두 번째 입력으로 매크로 녹화 종료
1. `6q` 녹화된 매크로 시퀀스를 6회 반복
