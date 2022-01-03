package com.svenschmidt.kitana

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.databinding.DataBindingUtil
import androidx.lifecycle.ViewModelProvider
import com.svenschmidt.kitana.databinding.ActivityDateTimeBinding
import com.svenschmidt.kitana.viewmodel.DateTimeViewModel

class DateTimeActivity : AppCompatActivity() {

    lateinit var viewModel: DateTimeViewModel
    lateinit var binding: ActivityDateTimeBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = DataBindingUtil.setContentView(this, R.layout.activity_date_time)
        binding.lifecycleOwner = this

        viewModel = ViewModelProvider(this).get(DateTimeViewModel::class.java)
        binding.viewmodel = viewModel
    }

}