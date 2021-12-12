# 本项目使用的 diff 规范

## 概述

这套 diff 规范名为 `area-diff`，基于 `git-diff` 修改而来，旨在精确描述新旧区划代码间的对应关系。这套规范的制定始终遵从以下四点原则：

- 逻辑严密：以行政区域的相交来定义新旧区划代码间的对应关系；
- 语法简洁：标识符均为单个字符，在保证表意清晰的前提下尽量少；
- 易于维护：语法符合直觉，使用区划名称表示对应关系，便于数据的录入和校对；
- 利于分析：利用处理后的数据，通过不太复杂的算法能够分析出代码间的对应关系并清晰地呈现。

## 背景

1. `data` 目录下的每一个文件称为一个**区划代码数据表**，简称**数据表**。数据表的每一行称为一条**区划代码记录**，简称**记录**。一条记录由**代码**和**名称**组成，由一个制表符 `\t` 隔开。每一条记录都与一定范围的行政区域相对应，称为**该记录对应的区域**；
1. `diff` 目录下的每一个文件称为一个**差异表**，包含相应的两个数据表中记录的差异，其中作为相对参考的数据表称为**源表**，与之相对的数据表称为**目标表**。差异表的**原始内容**是通过对相应的数据表执行 `git diff -U0 --no-index` 并去除无关行后得到的。差异表中内容与原始内容完全一致的行称为**原始行**。

## 详细规则

1. 差异表的每一非空行分为四种类型，由其首字符决定，分别为：**删除行** (`-`), **增加行** (`+`), **内部变更行** (`=`) 及**注释行**（其他字符）。其中，内部变更行和非原始行的删除行、增加行统称为**变更行**；
1. 对差异表的任何修改都应当遵守以下规则：
    - 除非对相应的数据表进行了订正，否则**不能**删除差异表中包含原始内容的行，修改其中的原始内容，或向其中添加删除行或增加行；
    - 修改差异表中包含原始内容的行时，**应当**将其中的原始内容始终保持在行首，只修改原始内容之后到行尾的内容；
    - 差异表中的所有行的语法**必须**严格遵守随后的语法规范；
    - 差异表中的所有行不区分先后，因此可以任意排序，但需注意保持一定的结构。
1. 每一变更行在相应的数据表中都有其对应的记录，称为**该行的记录**。删除行的记录位于源表中。增加行的记录位于目标表中。内部变更行的记录同时位于源表和目标表中；
1. 变更行由其首字符、其记录和其**属性**依序连接而成。变更行的属性具有确定的语法，用于说明该行的记录与相关记录所对应的区域的相交关系。下文中讨论一行的属性时，均假定该行为变更行；
1. 删除（增加）行的属性，指定目标（源）表中所有**对应的区域与该行的记录对应的区域相交**的同级记录（若有）或上一级记录（若没有这样的同级记录）；
1. 删除行的属性以字符 `>` 起始，后接一个或多个以字符 `,` 分隔的**记录选择器**。增加行的属性以字符 `<` 起始，其余部分的语法与删除行相同。内部变更行的属性语法，要么与删除行相同，要么与增加行相同；
1. 通过改变内部变更行的首字符，可得到与之对应的删除行或增加行。内部变更行的属性所指定的记录，是从其对应的删除行或增加行的属性所指定的记录中去除该行自身的记录后的结果；
1. 记录选择器有五种类型，分别为：**指定名称**、**当前名称**、**当前代码**、**父代码**和**存疑**。其中，前四种选择器统称为**普通**选择器。各选择器的定义如下：
    - **指定名称**选择器的值为文本，选择名称与其值相同的，与当前行的记录距离最小的记录；
    - **当前名称**选择器的值为单个字符 `#`，选择与当前行的记录具有相同名称且距离最小的记录；
    - **当前代码**选择器的值为单个字符 `.`，选择与当前行的记录具有相同代码的记录；
    - **父代码**选择器的值为两个字符 `..`，选择当前行的记录的代码的父代码所对应的记录；
    - **存疑**选择器的值由普通选择器的值后接一**存疑标志**组成。存疑标志的可用值为单个字符 `?` 或 `!`，其中 `?` 指示该选择器无效并将其禁用，一般用于找不到相关记录时；而 `!` 指示该选择器有效，其选择的记录与相应的普通选择器相同，一般用于能找到相关记录，且有充分的理由启用该选择器，但没有官方解释时。
1. 两条记录的距离的可能取值为：
    - 0（用户手动选择）
    - 1（二级行政区相同）
    - 2（一级行政区相同，但二级行政区不同）
    - 3（一级行政区不同）

    遵守本规范的实现，在选择与指定记录距离最小的记录时，应首先自动判断这样的记录是否唯一，若唯一，则选择该唯一的记录；若不唯一，则将其交由用户选择。

## 注意事项

1. 在将区划变更情况的文字描述录入差异表时，需要额外考虑内部变更的存在，这可以通过在页面中搜索关键词“划归”并手动筛选得到。每一年的差异表修改完毕后，应当在 [diff-note.md](diff-note.md) 中记录该年中所有的内部变更，若没有则记录“无”。

## 例子

- 撤销崇川区、港闸区，设立新的南通市崇川区，以原崇川区、港闸区的行政区域为新的崇川区的行政区域。

    ```diff
    -320602	崇川区>#
    -320611	港闸区>崇川区
    +320613	崇川区<#,港闸区
    ```

- 邢台市桥西区更名为信都区。

    ```diff
    -130503	桥西区>.
    +130503	信都区<.
    ```

- 将磁县高臾镇、光禄镇、辛庄营乡、花官营乡、台城乡划归邯郸市邯山区管辖，将磁县林坛镇、南城乡划归邯郸市复兴区管辖。

    将巴音郭楞蒙古自治州和静县、焉耆回族自治县、博湖县、和硕县、若羌县、且末县的部分区域划归铁门关市管辖。

    ```diff
    =130427	磁县>邯山区,复兴区
    =659006	铁门关市<和静县,焉耆回族自治县,博湖县,和硕县,若羌县,且末县
    ```

- 更多的例子：参见已完成的差异表（尤其是 2005 年）和相应的文字描述。