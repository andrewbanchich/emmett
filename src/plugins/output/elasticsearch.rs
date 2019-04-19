/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-output-elasticsearch.html

#[derive(Debug)]
pub struct ElasticsearchOutput {
    action: Option<String>,
    bulk_path: Option<bool>,
    cacert: Option<String>,
    custom_headers: Option<String>,
    doc_as_upsert: Option<bool>,
    document_id: Option<bool>,
    document_type: Option<bool>,
    failure_type_logging_whitelist: Option<bool>,
    healthcheck_path: Option<bool>,
    hosts: Option<bool>,
    http_compression: Option<bool>,
    ilm_enabled: Option<bool>,
    ilm_pattern: Option<bool>,
    ilm_policy: Option<bool>,
    ilm_rollover_alias: Option<bool>,
    index: Option<bool>,
    keystore: Option<bool>,
    keystore_password: Option<bool>,
    manage_template: Option<bool>,
    parameters: Option<bool>,
    parent: Option<bool>,
    password: Option<bool>,
    path: Option<bool>,
    pipeline: Option<bool>,
    pool_max: Option<bool>,
    pool_max_per_route: Option<bool>,
    proxy: Option<bool>,
    resurrect_delay: Option<bool>,
    retry_initial_interval: Option<bool>,
    retry_max_interval: Option<bool>,
    retry_on_conflict: Option<bool>,
    routing: Option<bool>,
    script: Option<bool>,
    script_lang: Option<bool>,
    script_type: Option<bool>,
    script_var_name: Option<bool>,
    scripted_upsert: Option<bool>,
    sniffing: Option<bool>,
    sniffing_delay: Option<bool>,
    sniffing_path: Option<bool>,
    ssl: Option<bool>,
    ssl_certificate_verification: Option<bool>,
    template: Option<bool>,
    template_name: Option<bool>,
    template_overwrite: Option<bool>,
    timeout: Option<bool>,
    truststore: Option<bool>,
    truststore_password: Option<bool>,
    upsert: Option<bool>,
    user: Option<bool>,
    validate_after_inactivity: Option<bool>,
    version: Option<bool>,
    version_type: Option<bool>
}

impl Default for ElasticsearchOutput {

    fn default() -> ElasticsearchOutput {
        ElasticsearchOutput {
            action: None,
            bulk_path: None,
            cacert: None,
            custom_headers: None,
            doc_as_upsert: None,
            document_id: None,
            document_type: None,
            failure_type_logging_whitelist: None,
            healthcheck_path: None,
            hosts: None,
            http_compression: None,
            ilm_enabled: None,
            ilm_pattern: None,
            ilm_policy: None,
            ilm_rollover_alias: None,
            index: None,
            keystore: None,
            keystore_password: None,
            manage_template: None,
            parameters: None,
            parent: None,
            password: None,
            path: None,
            pipeline: None,
            pool_max: None,
            pool_max_per_route: None,
            proxy: None,
            resurrect_delay: None,
            retry_initial_interval: None,
            retry_max_interval: None,
            retry_on_conflict: None,
            routing: None,
            script: None,
            script_lang: None,
            script_type: None,
            script_var_name: None,
            scripted_upsert: None,
            sniffing: None,
            sniffing_delay: None,
            sniffing_path: None,
            ssl: None,
            ssl_certificate_verification: None,
            template: None,
            template_name: None,
            template_overwrite: None,
            timeout: None,
            truststore: None,
            truststore_password: None,
            upsert: None,
            user:None,
            validate_after_inactivity:None,
            version: None,
            version_type: None
        }
    }

}

impl ElasticsearchOutput {

    pub fn new() -> ElasticsearchOutput {
        ElasticsearchOutput {
            ..Default::default()
        }
    }
    
}
