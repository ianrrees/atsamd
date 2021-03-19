var searchIndex = JSON.parse('{\
"samd_dma":{"doc":"DMA library for Microchip SAM micro-controllers.","i":[[3,"Channel","samd_dma","DMA channel.",null,null],[3,"Channels","","A bitfield of possible channels.",null,null],[3,"Interrupts","","A bitfield to represent channel interrupt flags.",null,null],[3,"TransferDescriptor","","The raw descriptor memory structure used by the DMA system…",null,null],[3,"DMAController","","DMA system controller.",null,null],[4,"TransactionError","","Error type for the kinds of errors that can occur during a…",null,null],[13,"InvalidDescriptor","","An invalid descriptor was fetched from memory.",0,null],[13,"TransferError","","A bus error was detected during a beat transfer.",0,null],[13,"CRCError","","The CRC module detected data corruption.",0,null],[4,"WaitResult","","The return value of `Transaction::try_wait()`.",null,null],[13,"Done","","The transaction has ended or been aborted.",1,null],[13,"Suspended","","The transaction is suspended.",1,null],[13,"Ongoing","","The transaction is still ongoing.",1,null],[4,"Status","","The status of a channel.",null,null],[13,"Busy","","",2,null],[13,"Pending","","",2,null],[13,"FetchError","","",2,null],[4,"Priority","","Priority level of a channel.",null,null],[13,"Level0","","",3,null],[13,"Level1","","",3,null],[13,"Level2","","",3,null],[13,"Level3","","",3,null],[4,"QoS","","Quality of Service guarantee for the DMA system.",null,null],[13,"Disable","","",4,null],[13,"Low","","",4,null],[13,"Medium","","",4,null],[13,"Critical","","",4,null],[4,"TriggerAction","","What action occurs when a trigger is received.",null,null],[13,"Block","","Trigger starts a block transfer.",5,null],[13,"Beat","","Trigger starts a beat transfer.",5,null],[13,"Transaction","","Trigger starts a transaction transfer.",5,null],[4,"TriggerSource","","Trigger source for a channel.",null,null],[13,"Disable","","",6,null],[13,"RtcTimestamp","","",6,null],[13,"DsuDcc0","","",6,null],[13,"DsuDcc1","","",6,null],[13,"Sercom0Rx","","",6,null],[13,"Sercom0Tx","","",6,null],[13,"Sercom1Rx","","",6,null],[13,"Sercom1Tx","","",6,null],[13,"Sercom2Rx","","",6,null],[13,"Sercom3Tx","","",6,null],[13,"Sercom4Rx","","",6,null],[13,"Sercom4Tx","","",6,null],[13,"Sercom5Rx","","",6,null],[13,"Sercom5Tx","","",6,null],[13,"Sercom6Rx","","",6,null],[13,"Sercom6Tx","","",6,null],[13,"Sercom7Rx","","",6,null],[13,"Sercom7Tx","","",6,null],[13,"Can0Debug","","",6,null],[13,"Can1Debug","","",6,null],[13,"Tcc0Ovf","","",6,null],[13,"Tcc0Mc0","","",6,null],[13,"Tcc0Mc1","","",6,null],[13,"Tcc0Mc2","","",6,null],[13,"Tcc0Mc3","","",6,null],[13,"Tcc0Mc4","","",6,null],[13,"Tcc0Mc5","","",6,null],[13,"Tcc1Ovf","","",6,null],[13,"Tcc1Mc0","","",6,null],[13,"Tcc1Mc1","","",6,null],[13,"Tcc1Mc2","","",6,null],[13,"Tcc1Mc3","","",6,null],[13,"Tcc2Ovf","","",6,null],[13,"Tcc2Mc0","","",6,null],[13,"Tcc2Mc1","","",6,null],[13,"Tcc2Mc2","","",6,null],[13,"Tcc3Ovf","","",6,null],[13,"Tcc3Mc0","","",6,null],[13,"Tcc3Mc1","","",6,null],[13,"Tcc4Ovf","","",6,null],[13,"Tcc4Mc0","","",6,null],[13,"Tcc4Mc1","","",6,null],[13,"Tc0Ovf","","",6,null],[13,"Tc0Mc0","","",6,null],[13,"Tc0Mc1","","",6,null],[13,"Tc1Ovf","","",6,null],[13,"Tc1Mc0","","",6,null],[13,"Tc1Mc1","","",6,null],[13,"Tc2Ovf","","",6,null],[13,"Tc2Mc0","","",6,null],[13,"Tc2Mc1","","",6,null],[13,"Tc3Ovf","","",6,null],[13,"Tc3Mc0","","",6,null],[13,"Tc3Mc1","","",6,null],[13,"Tc4Ovf","","",6,null],[13,"Tc4Mc0","","",6,null],[13,"Tc4Mc1","","",6,null],[13,"Tc5Ovf","","",6,null],[13,"Tc5Mc0","","",6,null],[13,"Tc5Mc1","","",6,null],[13,"Tc6Ovf","","",6,null],[13,"Tc6Mc0","","",6,null],[13,"Tc6Mc1","","",6,null],[13,"Tc7Ovf","","",6,null],[13,"Tc7Mc0","","",6,null],[13,"Tc7Mc1","","",6,null],[13,"Adc0ResRdy","","",6,null],[13,"Adc0Seq","","",6,null],[13,"Adc1ResRdy","","",6,null],[13,"Adc1Seq","","",6,null],[13,"Dac0Empty","","",6,null],[13,"Dac1Empty","","",6,null],[13,"Dac0ResRdy","","",6,null],[13,"Dac1ResRdy","","",6,null],[13,"I2sRx0","","",6,null],[13,"I2sRx1","","",6,null],[13,"I2sTx0","","",6,null],[13,"I2sTx1","","",6,null],[13,"PccRx","","",6,null],[13,"AesWr","","",6,null],[13,"AesRd","","",6,null],[13,"QspiRx","","",6,null],[13,"QspiTx","","",6,null],[4,"EventOutput","","When EVSYS events should be output.",null,null],[13,"Disable","","",7,null],[13,"Block","","",7,null],[13,"Beat","","",7,null],[4,"BlockAction","","Define what happens when a block transfer completes.",null,null],[13,"NoAct","","Channel will be disabled if this is the last block transfer.",8,null],[13,"Int","","Block interrupt will be generated, plus the action of NoAct.",8,null],[13,"Suspend","","Channel will be suspended.",8,null],[13,"Both","","Channel will be suspended and block interrupt will be…",8,null],[4,"BeatSize","","Size of a DMA beat transfer memory access.",null,null],[13,"Byte","","",9,null],[13,"HWord","","",9,null],[13,"Word","","",9,null],[4,"StepSize","","Size of the address advancement step.",null,null],[13,"X1","","",10,null],[13,"X2","","",10,null],[13,"X4","","",10,null],[13,"X8","","",10,null],[13,"X16","","",10,null],[13,"X32","","",10,null],[13,"X64","","",10,null],[13,"X128","","",10,null],[11,"id","","Return the channel ID.",11,[[]]],[11,"set_trigger_action","","Set the trigger action for the channel.",11,[[["triggeraction",4]]]],[11,"get_trigger_action","","Get the trigger action for the channel.",11,[[],["triggeraction",4]]],[11,"set_source","","Set the source trigger for the DMA Channel.",11,[[["triggersource",4]]]],[11,"get_source","","Get the trigger source for the channel.",11,[[],["triggersource",4]]],[11,"set_priority","","Set the priority level of the channel.",11,[[["priority",4]]]],[11,"get_priority","","Get channel priority level.",11,[[],["priority",4]]],[11,"get_first_descriptor","","Get a mutable reference to the first descriptor for the…",11,[[],["transferdescriptor",3]]],[11,"get_interrupt_flags","","Get the channel\'s interrupt flags.",11,[[],["interrupts",3]]],[11,"clear_interrupt_flags","","Reset the channel\'s interrupt flags.",11,[[]]],[11,"enable_interrupts","","Enable interrupts for the channel. Any interrupts that are…",11,[[["interrupts",3]]]],[11,"get_enabled_interrupts","","Get the set of enabled channel interrupts.",11,[[],["interrupts",3]]],[11,"get_writeback_descriptor","","Read descriptor from the Write-back Address of this channel.",11,[[]]],[11,"enable","","Enable the DMA channel.",11,[[]]],[11,"is_enabled","","Return whether the channel is enabled or not.",11,[[]]],[11,"reset","","Reset the DMA channel. This will set all channel registers…",11,[[]]],[11,"trigger","","Manually trigger the channel.",11,[[]]],[11,"suspend","","Suspend the ongoing transaction. Returns `true` if the…",11,[[]]],[11,"resume","","Resume the ongoing transaction. Returns `true` if command…",11,[[]]],[11,"disable","","Disable the channel. This aborts any ongoing transaction.",11,[[]]],[11,"is_pending","","Returns `true` if a transfer is pending on the channel.…",11,[[]]],[11,"is_busy","","Returns `true` if the channel has started a transfer.…",11,[[]]],[11,"poll_status","","Poll the channel to determine the status of the transaction.",11,[[],[["waitresult",4],["result",4],["transactionerror",4]]]],[18,"CHAN0","","",12,null],[18,"CHAN1","","",12,null],[18,"CHAN2","","",12,null],[18,"CHAN3","","",12,null],[18,"CHAN4","","",12,null],[18,"CHAN5","","",12,null],[18,"CHAN6","","",12,null],[18,"CHAN7","","",12,null],[18,"CHAN8","","",12,null],[18,"CHAN9","","",12,null],[18,"CHAN10","","",12,null],[18,"CHAN11","","",12,null],[11,"empty","","Returns an empty set of flags",12,[[],["channels",3]]],[11,"all","","Returns the set containing all flags.",12,[[],["channels",3]]],[11,"bits","","Returns the raw value of the flags currently stored.",12,[[]]],[11,"from_bits","","Convert from underlying bit representation, unless that…",12,[[],[["option",4],["channels",3]]]],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any…",12,[[],["channels",3]]],[11,"from_bits_unchecked","","Convert from underlying bit representation, preserving all…",12,[[],["channels",3]]],[11,"is_empty","","Returns `true` if no flags are currently stored.",12,[[]]],[11,"is_all","","Returns `true` if all flags are currently set.",12,[[]]],[11,"intersects","","Returns `true` if there are flags common to both `self`…",12,[[["channels",3]]]],[11,"contains","","Returns `true` all of the flags in `other` are contained…",12,[[["channels",3]]]],[11,"insert","","Inserts the specified flags in-place.",12,[[["channels",3]]]],[11,"remove","","Removes the specified flags in-place.",12,[[["channels",3]]]],[11,"toggle","","Toggles the specified flags in-place.",12,[[["channels",3]]]],[11,"set","","Inserts or removes the specified flags depending on the…",12,[[["channels",3]]]],[18,"TERR","","",13,null],[18,"TCMPL","","",13,null],[18,"SUSP","","",13,null],[11,"empty","","Returns an empty set of flags",13,[[],["interrupts",3]]],[11,"all","","Returns the set containing all flags.",13,[[],["interrupts",3]]],[11,"bits","","Returns the raw value of the flags currently stored.",13,[[]]],[11,"from_bits","","Convert from underlying bit representation, unless that…",13,[[],[["option",4],["interrupts",3]]]],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any…",13,[[],["interrupts",3]]],[11,"from_bits_unchecked","","Convert from underlying bit representation, preserving all…",13,[[],["interrupts",3]]],[11,"is_empty","","Returns `true` if no flags are currently stored.",13,[[]]],[11,"is_all","","Returns `true` if all flags are currently set.",13,[[]]],[11,"intersects","","Returns `true` if there are flags common to both `self`…",13,[[["interrupts",3]]]],[11,"contains","","Returns `true` all of the flags in `other` are contained…",13,[[["interrupts",3]]]],[11,"insert","","Inserts the specified flags in-place.",13,[[["interrupts",3]]]],[11,"remove","","Removes the specified flags in-place.",13,[[["interrupts",3]]]],[11,"toggle","","Toggles the specified flags in-place.",13,[[["interrupts",3]]]],[11,"set","","Inserts or removes the specified flags depending on the…",13,[[["interrupts",3]]]],[11,"new","","Create a new empty descriptor.",14,[[],["transferdescriptor",3]]],[11,"get_src_addr","","Get the type-erased source address.",14,[[]]],[11,"get_dst_addr","","Get the type-erased destination address.",14,[[]]],[11,"get_next_desc_addr","","Get address for the next linked descriptor.",14,[[]]],[11,"set_src_addr","","Set the source address of the descriptor. This is a type…",14,[[]]],[11,"set_dst_addr","","Set the destination address of the descriptor. This is a…",14,[[]]],[11,"set_valid","","Mark the descriptor as valid.",14,[[]]],[11,"set_invalid","","Mark the descriptor as invalid.",14,[[]]],[11,"is_valid","","Return the value of the valid bit.",14,[[]]],[11,"set_block_count","","Set the block count for the descriptor.",14,[[]]],[11,"get_block_transfer_count","","Get the configured block transfer count.",14,[[]]],[11,"get_step_size","","Get the step size of the descriptor.",14,[[],["stepsize",4]]],[11,"set_step_size","","Set the step size for the descriptor.",14,[[["stepsize",4]]]],[11,"get_step_selection","","Get which address is incremented with the descriptor\'s…",14,[[]]],[11,"set_step_selection","","Set which address is incremented with the descriptor\'s…",14,[[]]],[11,"get_dest_addr_increment","","Get whether the destination address is incremented after…",14,[[]]],[11,"set_dest_addr_increment","","Set whether the destination address is incremented after…",14,[[]]],[11,"get_src_addr_increment","","Get whether the source address is incremented after each…",14,[[]]],[11,"set_src_addr_increment","","Set whether the source address is incremented after each…",14,[[]]],[11,"get_block_action","","Get the action taken after this block transfer completes.",14,[[],["blockaction",4]]],[11,"set_block_action","","Set the action taken after this block transfer completes.",14,[[["blockaction",4]]]],[11,"get_event_output","","Get the trigger that causes the transfer to output an event.",14,[[],["eventoutput",4]]],[11,"set_event_output","","Set the trigger that causes the transfer to output an event.",14,[[["eventoutput",4]]]],[11,"link_descriptor","","Link a transfer descriptor to execute AFTER this descriptor.",14,[[]]],[11,"unlink_descriptor","","Unlink the next transfer descriptor, returning its address…",14,[[]]],[0,"storage","","Storage for DMA system base and write-back addresses.",null,null],[3,"Storage12","samd_dma::storage","Storage type for base and write-back memory.",null,null],[3,"Storage11","","Storage type for base and write-back memory.",null,null],[3,"Storage10","","Storage type for base and write-back memory.",null,null],[3,"Storage9","","Storage type for base and write-back memory.",null,null],[3,"Storage8","","Storage type for base and write-back memory.",null,null],[3,"Storage7","","Storage type for base and write-back memory.",null,null],[3,"Storage6","","Storage type for base and write-back memory.",null,null],[3,"Storage5","","Storage type for base and write-back memory.",null,null],[3,"Storage4","","Storage type for base and write-back memory.",null,null],[3,"Storage3","","Storage type for base and write-back memory.",null,null],[3,"Storage2","","Storage type for base and write-back memory.",null,null],[3,"Storage1","","Storage type for base and write-back memory.",null,null],[3,"UnsafeStorage","","A user allocated storage type.",null,null],[6,"CHANMAX","","The maximum amount of channels that can exist.",null,null],[8,"DmaStorage","","Trait for accessing the base and write-back addresses of a…",null,null],[16,"Size","","The number of channels supported.",15,null],[10,"baseaddr","","Get the address for the base descriptor memory.",15,[[]]],[10,"wbaddr","","Get the address for the write-back descriptor memory.",15,[[]]],[11,"new","","Create a custom DMA memory storage unit.",16,[[],["unsafestorage",3]]],[11,"into_inner","","Get back the memory pointers to the base and write-back…",16,[[]]],[0,"consts","samd_dma","Contains types used to identify DMA channels.",null,null],[6,"CH0","samd_dma::consts","",null,null],[6,"CH1","","",null,null],[6,"CH2","","",null,null],[6,"CH3","","",null,null],[6,"CH4","","",null,null],[6,"CH5","","",null,null],[6,"CH6","","",null,null],[6,"CH7","","",null,null],[6,"CH8","","",null,null],[6,"CH9","","",null,null],[6,"CH10","","",null,null],[6,"CH11","","",null,null],[11,"init","samd_dma","Initialise the DMA Controller with the specified storage.",17,[[["dmac",3]],["dmacontroller",3]]],[11,"disable","","Disable all channels and the CRC module. This will abort…",17,[[]]],[11,"enable","","Enable the DMA system.",17,[[]]],[11,"is_enabled","","Returns true if the DMA system is enabled.",17,[[]]],[11,"get_run_while_debug","","Get the value of the \\\"Run While Debug\\\" setting.",17,[[]]],[11,"set_run_while_debug","","Set the value of the \\\"Run While Debug\\\" setting.",17,[[]]],[11,"take_channel","","Take a DMA channel. If the channel has already been taken…",17,[[],[["channel",3],["option",4]]]],[11,"return_channel","","Return a channel to the controller. This will disable the…",17,[[["channel",3]]]],[11,"enable_priority_level","","Allow channels with the corresponding priority level to be…",17,[[["priority",4]]]],[11,"diable_priority_level","","Deny channels with the corresponding priority level to be…",17,[[["priority",4]]]],[11,"get_data_transfer_qos","","Get the Quality of Service guarantee for data transfer.",17,[[],["qos",4]]],[11,"get_fetch_qos","","Get the Quality of Service guarantee for fetching transfer…",17,[[],["qos",4]]],[11,"get_write_back_qos","","Get the Quality of Service guarantee for writing transfer…",17,[[],["qos",4]]],[11,"set_data_transfer_qos","","Set the Quality of Service guarantee for data transfer.",17,[[["qos",4]]]],[11,"set_fetch_qos","","Set the Quality of Service guarantee for fetching transfer…",17,[[["qos",4]]]],[11,"set_write_back_qos","","Set the Quality of Service guarantee for writing transfer…",17,[[["qos",4]]]],[11,"priority_level_enabled","","Return true if the priority level is enabled.",17,[[["priority",4]]]],[11,"set_priority_level_scheduling","","Enable or disable round-robin scheduling method for…",17,[[["priority",4]]]],[11,"get_channel_interrupt_status","","Get the interrupt status of all channels.",17,[[],["channels",3]]],[11,"get_pending_channels","","Get a bitfield of all pending channels.",17,[[],["channels",3]]],[11,"get_busy_channels","","Get a bitfield of all busy channels.",17,[[],["channels",3]]],[11,"get_active_channel","","Get ID of the last channel to be granted access to the DMA…",17,[[]]],[11,"trigger_channel","","Send a trigger request to a channel.",17,[[]]],[11,"get_active_block_transfer_count","","Get the block transfer count of the currently active…",17,[[],["option",4]]],[11,"priority_level_has_request","","Return whether the selected priority level has an active…",17,[[["priority",4]]]],[11,"get_lowest_pending_channel_interrupts","","Get the lowest pending interrupt channel\'s interrupt…",17,[[],["option",4]]],[11,"get_lowest_pending_channel_status","","Get the lowest pending interrupt channel\'s status, if…",17,[[],["option",4]]],[11,"get_channel_pending_interrupts","","Get the interrupt flags of a particular channel.",17,[[],["interrupts",3]]],[11,"set_channel_pending_interrupts","","Set the interrupt flags of a particular channel.",17,[[["interrupts",3]]]],[11,"get_channel_status","","Get the status of a particular channel.",17,[[],[["status",4],["option",4]]]],[11,"from","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"into","","",11,[[]]],[11,"try_into","","",11,[[],["result",4]]],[11,"borrow_mut","","",11,[[]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"into","","",12,[[]]],[11,"try_into","","",12,[[],["result",4]]],[11,"borrow_mut","","",12,[[]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"from","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"into","","",13,[[]]],[11,"try_into","","",13,[[],["result",4]]],[11,"borrow_mut","","",13,[[]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"from","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"into","","",14,[[]]],[11,"try_into","","",14,[[],["result",4]]],[11,"borrow_mut","","",14,[[]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"into","","",17,[[]]],[11,"try_into","","",17,[[],["result",4]]],[11,"borrow_mut","","",17,[[]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"from","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"into","","",1,[[]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"into","","",2,[[]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"into","","",3,[[]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"into","","",4,[[]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"into","","",5,[[]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"into","","",6,[[]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"into","","",7,[[]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"into","","",8,[[]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"into","","",9,[[]]],[11,"try_into","","",9,[[],["result",4]]],[11,"borrow_mut","","",9,[[]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"into","","",10,[[]]],[11,"try_into","","",10,[[],["result",4]]],[11,"borrow_mut","","",10,[[]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","samd_dma::storage","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"into","","",18,[[]]],[11,"try_into","","",18,[[],["result",4]]],[11,"borrow_mut","","",18,[[]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"from","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"into","","",19,[[]]],[11,"try_into","","",19,[[],["result",4]]],[11,"borrow_mut","","",19,[[]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"from","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"into","","",20,[[]]],[11,"try_into","","",20,[[],["result",4]]],[11,"borrow_mut","","",20,[[]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"from","","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"into","","",21,[[]]],[11,"try_into","","",21,[[],["result",4]]],[11,"borrow_mut","","",21,[[]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"from","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"into","","",22,[[]]],[11,"try_into","","",22,[[],["result",4]]],[11,"borrow_mut","","",22,[[]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"from","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"into","","",23,[[]]],[11,"try_into","","",23,[[],["result",4]]],[11,"borrow_mut","","",23,[[]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"from","","",24,[[]]],[11,"borrow","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"into","","",24,[[]]],[11,"try_into","","",24,[[],["result",4]]],[11,"borrow_mut","","",24,[[]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"from","","",25,[[]]],[11,"borrow","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"into","","",25,[[]]],[11,"try_into","","",25,[[],["result",4]]],[11,"borrow_mut","","",25,[[]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"from","","",26,[[]]],[11,"borrow","","",26,[[]]],[11,"try_from","","",26,[[],["result",4]]],[11,"into","","",26,[[]]],[11,"try_into","","",26,[[],["result",4]]],[11,"borrow_mut","","",26,[[]]],[11,"type_id","","",26,[[],["typeid",3]]],[11,"from","","",27,[[]]],[11,"borrow","","",27,[[]]],[11,"try_from","","",27,[[],["result",4]]],[11,"into","","",27,[[]]],[11,"try_into","","",27,[[],["result",4]]],[11,"borrow_mut","","",27,[[]]],[11,"type_id","","",27,[[],["typeid",3]]],[11,"from","","",28,[[]]],[11,"borrow","","",28,[[]]],[11,"try_from","","",28,[[],["result",4]]],[11,"into","","",28,[[]]],[11,"try_into","","",28,[[],["result",4]]],[11,"borrow_mut","","",28,[[]]],[11,"type_id","","",28,[[],["typeid",3]]],[11,"from","","",29,[[]]],[11,"borrow","","",29,[[]]],[11,"try_from","","",29,[[],["result",4]]],[11,"into","","",29,[[]]],[11,"try_into","","",29,[[],["result",4]]],[11,"borrow_mut","","",29,[[]]],[11,"type_id","","",29,[[],["typeid",3]]],[11,"from","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"into","","",16,[[]]],[11,"try_into","","",16,[[],["result",4]]],[11,"borrow_mut","","",16,[[]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"baseaddr","","",18,[[]]],[11,"wbaddr","","",18,[[]]],[11,"baseaddr","","",19,[[]]],[11,"wbaddr","","",19,[[]]],[11,"baseaddr","","",20,[[]]],[11,"wbaddr","","",20,[[]]],[11,"baseaddr","","",21,[[]]],[11,"wbaddr","","",21,[[]]],[11,"baseaddr","","",22,[[]]],[11,"wbaddr","","",22,[[]]],[11,"baseaddr","","",23,[[]]],[11,"wbaddr","","",23,[[]]],[11,"baseaddr","","",24,[[]]],[11,"wbaddr","","",24,[[]]],[11,"baseaddr","","",25,[[]]],[11,"wbaddr","","",25,[[]]],[11,"baseaddr","","",26,[[]]],[11,"wbaddr","","",26,[[]]],[11,"baseaddr","","",27,[[]]],[11,"wbaddr","","",27,[[]]],[11,"baseaddr","","",28,[[]]],[11,"wbaddr","","",28,[[]]],[11,"baseaddr","","",29,[[]]],[11,"wbaddr","","",29,[[]]],[11,"baseaddr","","",16,[[]]],[11,"wbaddr","","",16,[[]]],[11,"from","samd_dma","",3,[[["prilvl_a",4]],["priority",4]]],[11,"from","","",4,[[["dqos_a",4]],["qos",4]]],[11,"from","","",4,[[["fqos_a",4]],["qos",4]]],[11,"from","","",4,[[["wrbqos_a",4]],["qos",4]]],[11,"from","","",5,[[["variant",4],["trigact_a",4]],["triggeraction",4]]],[11,"from","","",6,[[["trigsrc_a",4],["variant",4]],["triggersource",4]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",14,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"sub","","Returns the set difference of the two sets of flags.",12,[[["channels",3]],["channels",3]]],[11,"sub","","Returns the set difference of the two sets of flags.",13,[[["interrupts",3]],["interrupts",3]]],[11,"eq","","",12,[[["channels",3]]]],[11,"ne","","",12,[[["channels",3]]]],[11,"eq","","",13,[[["interrupts",3]]]],[11,"ne","","",13,[[["interrupts",3]]]],[11,"cmp","","",12,[[["channels",3]],["ordering",4]]],[11,"cmp","","",13,[[["interrupts",3]],["ordering",4]]],[11,"partial_cmp","","",12,[[["channels",3]],[["option",4],["ordering",4]]]],[11,"lt","","",12,[[["channels",3]]]],[11,"le","","",12,[[["channels",3]]]],[11,"gt","","",12,[[["channels",3]]]],[11,"ge","","",12,[[["channels",3]]]],[11,"partial_cmp","","",13,[[["interrupts",3]],[["option",4],["ordering",4]]]],[11,"lt","","",13,[[["interrupts",3]]]],[11,"le","","",13,[[["interrupts",3]]]],[11,"gt","","",13,[[["interrupts",3]]]],[11,"ge","","",13,[[["interrupts",3]]]],[11,"sub_assign","","Disables all flags enabled in the set.",12,[[["channels",3]]]],[11,"sub_assign","","Disables all flags enabled in the set.",13,[[["interrupts",3]]]],[11,"not","","Returns the complement of this set of flags.",12,[[],["channels",3]]],[11,"not","","Returns the complement of this set of flags.",13,[[],["interrupts",3]]],[11,"bitand","","Returns the intersection between the two sets of flags.",12,[[["channels",3]],["channels",3]]],[11,"bitand","","Returns the intersection between the two sets of flags.",13,[[["interrupts",3]],["interrupts",3]]],[11,"bitor","","Returns the union of the two sets of flags.",12,[[["channels",3]],["channels",3]]],[11,"bitor","","Returns the union of the two sets of flags.",13,[[["interrupts",3]],["interrupts",3]]],[11,"bitxor","","Returns the left flags, but with all the right flags…",12,[[["channels",3]],["channels",3]]],[11,"bitxor","","Returns the left flags, but with all the right flags…",13,[[["interrupts",3]],["interrupts",3]]],[11,"bitand_assign","","Disables all flags disabled in the set.",12,[[["channels",3]]]],[11,"bitand_assign","","Disables all flags disabled in the set.",13,[[["interrupts",3]]]],[11,"bitor_assign","","Adds the set of flags.",12,[[["channels",3]]]],[11,"bitor_assign","","Adds the set of flags.",13,[[["interrupts",3]]]],[11,"bitxor_assign","","Toggles the set of flags.",12,[[["channels",3]]]],[11,"bitxor_assign","","Toggles the set of flags.",13,[[["interrupts",3]]]],[11,"hash","","",12,[[]]],[11,"hash","","",13,[[]]],[11,"extend","","",12,[[["intoiterator",8]]]],[11,"extend","","",13,[[["intoiterator",8]]]],[11,"from_iter","","",12,[[["intoiterator",8]],["channels",3]]],[11,"from_iter","","",13,[[["intoiterator",8]],["interrupts",3]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"clone","","",12,[[],["channels",3]]],[11,"clone","","",13,[[],["interrupts",3]]],[11,"default","","Return `EventOutput::Disable`",7,[[]]],[11,"default","","Return `BlockAction::NoAct`",8,[[]]],[11,"default","","Return `StepSize::X1`",10,[[]]],[11,"default","","",14,[[]]],[11,"default","samd_dma::storage","",18,[[],["storage12",3]]],[11,"default","","",19,[[],["storage11",3]]],[11,"default","","",20,[[],["storage10",3]]],[11,"default","","",21,[[],["storage9",3]]],[11,"default","","",22,[[],["storage8",3]]],[11,"default","","",23,[[],["storage7",3]]],[11,"default","","",24,[[],["storage6",3]]],[11,"default","","",25,[[],["storage5",3]]],[11,"default","","",26,[[],["storage4",3]]],[11,"default","","",27,[[],["storage3",3]]],[11,"default","","",28,[[],["storage2",3]]],[11,"default","","",29,[[],["storage1",3]]]],"p":[[4,"TransactionError"],[4,"WaitResult"],[4,"Status"],[4,"Priority"],[4,"QoS"],[4,"TriggerAction"],[4,"TriggerSource"],[4,"EventOutput"],[4,"BlockAction"],[4,"BeatSize"],[4,"StepSize"],[3,"Channel"],[3,"Channels"],[3,"Interrupts"],[3,"TransferDescriptor"],[8,"DmaStorage"],[3,"UnsafeStorage"],[3,"DMAController"],[3,"Storage12"],[3,"Storage11"],[3,"Storage10"],[3,"Storage9"],[3,"Storage8"],[3,"Storage7"],[3,"Storage6"],[3,"Storage5"],[3,"Storage4"],[3,"Storage3"],[3,"Storage2"],[3,"Storage1"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);