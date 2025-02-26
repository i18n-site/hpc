postcss stylus ast

stylus不支持编译到css nesting只用基于缩进的语法

vim插件，css保存时自动转为基于缩进的stylus

web/dom 下面的css和js的自动绑定

如何覆盖

# 默认直接选择器
比如 .xxx
会自动生成

i-test > .xxx
避免覆盖容器下面的元素的样式

# :global(_)

根元素选择器

i-test

对应非容器的元素，
也可以用它来规避直接选择器

