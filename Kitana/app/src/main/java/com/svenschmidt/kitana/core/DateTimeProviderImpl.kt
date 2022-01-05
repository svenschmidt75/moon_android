package com.svenschmidt.kitana.core

import java.time.LocalDateTime
import java.util.*

interface DateTimeProvider {
    fun start(observer: Observer)
    fun stop()
    fun getCurrentLocalDateTime(): LocalDateTime
    fun setCurrentLocalDateTime(dateTime: LocalDateTime)
    fun getSystemLocalDateTime(): LocalDateTime
}

class DateTimeProviderImpl : DateTimeProvider, Observable() {

    companion object {
        const val SECOND: Long = 1000
    }

    private var isRunning = false
    private var timer = Timer()
    var dateTime: LocalDateTime

    init {
        dateTime = LocalDateTime.now()
    }

    override fun stop() {
        timer.cancel()
        isRunning = false
    }

    override fun getSystemLocalDateTime(): LocalDateTime {
        return LocalDateTime.now()
    }

    override fun getCurrentLocalDateTime(): LocalDateTime {
        return dateTime
    }

    override fun setCurrentLocalDateTime(dateTime: LocalDateTime) {
        this@DateTimeProviderImpl.dateTime = dateTime
    }

    override fun start(observer: Observer) {
        timer = Timer()
        isRunning = true
        timer.schedule(object : TimerTask() {
            override fun run() {
                dateTime = dateTime.plusSeconds(1)
                observer.update(this@DateTimeProviderImpl, dateTime)
            }

        }, 0, SECOND)
    }

}