package com.svenschmidt.kitana.activities

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.databinding.DataBindingUtil
import androidx.lifecycle.ViewModelProvider
import com.svenschmidt.kitana.R
import com.svenschmidt.kitana.databinding.ActivityMoonBinding
import com.svenschmidt.kitana.viewmodel.MoonActivityViewModel
import kotlinx.android.synthetic.main.activity_moon.*

class MoonActivity : AppCompatActivity() {

    lateinit var viewModel: MoonActivityViewModel
    lateinit var binding: ActivityMoonBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = DataBindingUtil.setContentView(this, R.layout.activity_moon)
        binding.lifecycleOwner = this

        viewModel = ViewModelProvider(this).get(MoonActivityViewModel::class.java)
        binding.viewmodel = viewModel

        setupActionBar()
    }

    private fun setupActionBar() {

        setSupportActionBar(moon_activity_toolbar)
        supportActionBar!!.setDisplayHomeAsUpEnabled(true)
        moon_activity_toolbar.setNavigationOnClickListener {
            onBackPressed()
        }
    }

}