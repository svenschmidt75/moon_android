package com.svenschmidt.kitana.core

import java.util.*

interface DateTimeProvider {
    fun start(observer: Observer)
    fun stop()

}

class DateTimeProviderImpl: DateTimeProvider, Observable() {

    companion object {
        const val SECOND: Long = 1000
    }

    private var isRunning = false
    private var timer = Timer()

    override fun stop() {
        timer.cancel()
        isRunning = false
    }

    override fun start(observer: Observer) {
        timer = Timer()
        isRunning = true
        timer.schedule(object : TimerTask() {
            override fun run() {
                val nowUTC = System.currentTimeMillis()
                observer.update(this@DateTimeProviderImpl, nowUTC)
            }

        }, 0, SECOND)
    }

}