package com.svenschmidt.kitana.core

import java.text.SimpleDateFormat
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter
import java.util.*
import java.util.Locale

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

    inner class Helper : TimerTask() {
        override fun run() {
            val c = Calendar.getInstance()
            val time = c.time.time

            val df = SimpleDateFormat("dd-MMM-yyyy HH:mm:ss", Locale.getDefault())
            val formattedDate: String = df.format(c)
            println("Current time => $formattedDate")

        }
    }

    override fun stop() {
        timer.cancel()
        isRunning = false
    }

    override fun start(observer: Observer) {
        timer = Timer()
        isRunning = true
        timer.schedule(object : TimerTask() {
            override fun run() {
                val now = LocalDateTime.now()

                val formatter = DateTimeFormatter.ofPattern("yyyy-MMM-dd HH:mm:ss.SSS")
                val formatted = now.format(formatter)
                println("Current time => $formatted")

                observer.update(this@DateTimeProviderImpl, formatted)
            }

        }, 0, SECOND)
    }

}