package com.svenschmidt.kitana.core

import java.time.LocalDateTime
import java.util.*

class DateTimeProviderImpl : DateTimeProvider {

    companion object {
        const val SECOND: Long = 1000
    }

    private var subscribers = mutableListOf<(dateTime: LocalDateTime) -> Unit>()
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

    override fun subscribe(observer: (dateTime: LocalDateTime) -> Unit): () -> Unit {
        subscribers.add(observer)
        return {
            subscribers.remove(observer)
        };
    }

    override fun start() {
        timer = Timer()
        isRunning = true
        timer.schedule(object : TimerTask() {
            override fun run() {
                dateTime = dateTime.plusSeconds(1)

                for (subscriber in subscribers) {
                    subscriber.invoke(dateTime)
                }
            }

        }, 0, SECOND)
    }

}