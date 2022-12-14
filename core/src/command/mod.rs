use serde::Serialize;
use crate::ogf::Ogf;

pub mod disconnect;
pub mod get_mws_transport_layer_configuration;
pub mod host_buffer_size;
pub mod host_number_of_completed_packets;
pub mod le_access_cis_request;
pub mod le_add_device_to_periodic_advertiser_list;
pub mod le_add_device_to_resolving_list;
pub mod le_add_device_to_white_list;
pub mod le_big_create_sync;
pub mod le_big_terminate_sync;
pub mod le_clear_advertising_sets;
pub mod le_clear_periodic_advertiser_list;
pub mod le_clear_resolving_list;
pub mod le_clear_white_list;
pub mod le_connection_cte_request_enable;
pub mod le_connection_cte_response_enable;
pub mod le_connection_update;
pub mod le_create_big;
pub mod le_create_big_test;
pub mod le_create_cis;
pub mod le_create_connection_cancel;
pub mod le_create_connection;
pub mod le_enable_encryption;
pub mod le_encrypt;
pub mod le_enhanced_read_transmit_power_level;
pub mod le_extended_create_connection;
pub mod le_generate_dhkey;
pub mod le_iso_read_test_counters;
pub mod le_iso_receive_test;
pub mod le_iso_test_end;
pub mod le_iso_transmit_test;
pub mod le_long_term_key_request_negative_reply;
pub mod le_long_term_key_request_reply;
pub mod le_modify_sleep_clock_accuracy;
pub mod le_periodic_advertising_create_sync_cancel;
pub mod le_periodic_advertising_create_sync;
pub mod le_periodic_advertising_set_info_transfer;
pub mod le_periodic_advertising_sync_transfer;
pub mod le_periodic_advertising_terminate_sync;
pub mod le_rand;
pub mod le_read_advertising_physical_channel_tx_power;
pub mod le_read_antenna_information;
pub mod le_read_buffer_size;
pub mod le_read_channel_map;
pub mod le_read_iso_link_quality;
pub mod le_read_iso_tx_sync;
pub mod le_read_local_p256_public_key;
pub mod le_read_local_resolvable_address;
pub mod le_read_local_supported_features;
pub mod le_read_maximum_advertising_data_length;
pub mod le_read_maximum_data_length;
pub mod le_read_number_of_supported_advertising_sets;
pub mod le_read_peer_resolvable_address;
pub mod le_read_periodic_advertiser_list_size;
pub mod le_read_phy;
pub mod le_read_remote_features;
pub mod le_read_remote_transmit_power_level;
pub mod le_read_resolving_list_size;
pub mod le_read_rf_path_compensation;
pub mod le_read_suggested_default_data_length;
pub mod le_read_supported_states;
pub mod le_read_transmit_power;
pub mod le_read_white_list_size;
pub mod le_receiver_test;
pub mod le_reject_cis_request;
pub mod le_remote_connection_parameter_request_negative_reply;
pub mod le_remote_connection_parameter_request_reply;
pub mod le_remove_advertising_set;
pub mod le_remove_cig_command;
pub mod le_remove_device_from_periodic_advertiser_list;
pub mod le_remove_device_from_resolving_list;
pub mod le_remove_device_from_white_list;
pub mod le_remove_iso_data_path;
pub mod le_request_peer_sca_command;
pub mod le_set_address_resolution_enable;
pub mod le_set_advertising_data;
pub mod le_set_advertising_enable;
pub mod le_set_advertising_parameters;
pub mod let_set_advertising_set_random_address;
pub mod le_set_cig_parameters;
pub mod le_set_cig_parameters_test;
pub mod le_set_connection_cte_receive_parameters;
pub mod le_set_connection_cte_transmit_parameters;
pub mod le_set_connectionless_cte_transmit_enable;
pub mod le_set_connectionless_cte_transmit_parameters;
pub mod le_set_connectionless_iq_sampling_enable;
pub mod le_set_data_length;
pub mod le_set_default_periodic_advertising_sync_transfer_parameters;
pub mod le_set_default_phy;
pub mod le_set_event_mask;
pub mod le_set_extended_advertising_data;
pub mod le_set_extended_advertising_enable;
pub mod le_set_extended_advertising_parameters;
pub mod le_set_extended_scan_enable;
pub mod le_set_extended_scan_parameters;
pub mod le_set_extended_scan_response_data;
pub mod le_set_host_channel_classification;
pub mod le_set_host_feature;
pub mod le_set_path_loss_reporting_enable;
pub mod le_set_path_loss_reporting_parameters;
pub mod le_set_periodic_advertising_data;
pub mod le_set_periodic_advertising_enable;
pub mod le_set_periodic_advertising_parameters;
pub mod le_set_periodic_advertising_receive_enable;
pub mod le_set_periodic_advertising_sync_transfer_parameters;
pub mod le_set_phy;
pub mod le_set_privacy_mode;
pub mod le_set_random_address;
pub mod le_set_resolvable_private_address_timeout;
pub mod le_set_scan_enable;
pub mod le_set_scan_parameters;
pub mod le_set_scan_response_data;
pub mod le_set_transmit_power_report_enable;
pub mod le_setup_iso_data_path;
pub mod le_terminate_big_command;
pub mod le_test_end;
pub mod le_transmitter_test;
pub mod le_write_rf_path_compensation;
pub mod le_write_suggested_default_data_length;
pub mod read_authenticated_payload_timeout;
pub mod read_bd_addr;
pub mod read_buffer_size;
pub mod read_le_host_support;
pub mod read_local_supported_codec_capabilities;
pub mod read_local_supported_codecs;
pub mod read_local_supported_commands;
pub mod read_local_supported_controller_delay;
pub mod read_local_supported_features;
pub mod read_local_version_information;
pub mod read_remote_version_information;
pub mod read_rssi;
pub mod read_transmit_power_level;
pub mod reset;
pub mod set_controller_to_host_flow_control;
pub mod set_event_mask;
pub mod set_event_mask_page_2;
pub mod set_mws_channel_parameters;
pub mod set_mws_scan_frequency_table;
pub mod set_mws_signalling;
pub mod set_mws_transport_layer;
pub mod write_authenticated_payload_timeout;
pub mod write_le_host_support;

pub trait Command {
    const OGF: Ogf;
    const OCF: u16;
    type Parameters: Serialize;
    type ReturnParameters;

    fn opcode(&self) -> u16 {
        (Self::OCF & 0b1111111111) | ((((Self::OGF as u8) & 0b111111) as u16) << 10)
    }

    fn parameters(&self) -> Self::Parameters;
}