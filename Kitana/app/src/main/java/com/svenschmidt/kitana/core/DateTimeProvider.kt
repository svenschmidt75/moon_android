package com.svenschmidt.kitana.core

import java.time.ZonedDateTime

interface DateTimeProvider {
    fun start()
    fun stop()
    fun subscribe(observer: (dateTime: ZonedDateTime) -> Unit): () -> Unit
    fun getCurrentLocalDateTime(): ZonedDateTime
    fun setCurrentLocalDateTime(dateTime: ZonedDateTime)
    fun getSystemLocalDateTime(): ZonedDateTime
}