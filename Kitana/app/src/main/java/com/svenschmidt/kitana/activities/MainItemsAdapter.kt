package com.svenschmidt.kitana.activities

import android.content.Context
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import com.svenschmidt.kitana.R
import com.svenschmidt.kitana.core.MainItemDesc
import kotlinx.android.synthetic.main.main_cardview_item.view.*

class MainItemsAdapter(private val context: Context, private val items: ArrayList<MainItemDesc>) : RecyclerView.Adapter<RecyclerView.ViewHolder>() {

    private class MyViewHolder(view: View) : RecyclerView.ViewHolder(view)

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): RecyclerView.ViewHolder {
        return MyViewHolder(LayoutInflater.from(context).inflate(R.layout.main_cardview_item, parent, false))
    }

    override fun onBindViewHolder(holder: RecyclerView.ViewHolder, position: Int) {
        val model = items[position]

        holder.itemView.tv_description.text = model.text
        holder.itemView.tv_image.setImageResource(model.imageId)

//        holder.itemView.setOnClickListener {
//            if (onClickListener != null) {
//                onClickListener!!.onClick(position, model)
//            }
//        }
    }

    override fun getItemCount(): Int {
        return items.size
    }

}
