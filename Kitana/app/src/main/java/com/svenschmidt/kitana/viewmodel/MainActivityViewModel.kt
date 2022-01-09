package com.svenschmidt.kitana.viewmodel

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import com.svenschmidt.kitana.R
import com.svenschmidt.kitana.core.MainItemDesc

class MainActivityViewModel(application: Application) : AndroidViewModel(application) {

    fun getMainItems(): ArrayList<MainItemDesc> {
        val mainItems: ArrayList<MainItemDesc> = ArrayList()

        mainItems.add(MainItemDesc(0, R.drawable.moon2, "Moon"))
        mainItems.add(MainItemDesc(1, R.drawable.moon2, "Moon Animation"))

        return mainItems
    }

}