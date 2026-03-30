# GCC Roulette Size

`охаё люблители поковырять Linux`

пред вами программа которая сравнивает по размеру бинарники собранные из кода на C с разными параметрами оптимизации. здесь используется GCC в качестве компилятора C.

сборку там сами осилите.

## запуск

```shell
cargo run script.c
```

и там вам выведет какая оптимизация топ

## пример вывода (test.c, он есть в корне проекта для теста)

```shell
❯ ./target/release/GCCRouletteSize test.c
compilation results:
--------------------------------------------------
-O0 optimization: gcc-O0 (size: 16256 bytes)
-O1 optimization: gcc-O1 (size: 16232 bytes)
-O2 optimization: gcc-O2 (size: 16232 bytes)
-O3 optimization: gcc-O3 (size: 16232 bytes)
-Os optimization: gcc-Os (size: 16224 bytes)
-Oz optimization: gcc-Oz (size: 16224 bytes)

==================================================
Summary:
  Best optimizations: -Os, -Oz
  Size: 16224 bytes

Size comparison:
  -O0 :    16256 bytes +32 bytes
  -O1 :    16232 bytes +8 bytes
  -O2 :    16232 bytes +8 bytes
  -O3 :    16232 bytes +8 bytes
  -Os :    16224 bytes best
  -Oz :    16224 bytes best
==================================================
```

## из минусов

пока что программа может работать только с одним файлом и только с бибилиотеками, которые не нужно указывать компилятору

всё, всем пака, `i use brain btw`
