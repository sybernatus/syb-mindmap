package com.sybernatus.sybmindmap.data

data class Node(
    var image: Image? = null,
    var children: List<Node>? = null
)
