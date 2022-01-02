package com.svenschmidt.kitana

import android.content.Context
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.AttributeSet
import android.view.View
import androidx.lifecycle.ViewModelProvider
import com.svenschmidt.kitana.viewmodel.DateTimeViewModel

class DateTimeActivity : AppCompatActivity() {

    lateinit var viewModel: DateTimeViewModel

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_date_time)

        viewModel = ViewModelProvider(this).get(DateTimeViewModel::class.java)
    }

}