use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Gathers Clusters commands")]
pub struct Command {
    #[structopt(subcommand)]
    pub command: CommandChoices,
}

#[derive(Debug, StructOpt)]
pub enum CommandChoices {
    List(ListClusters),
    Create(CreateCluster),
    Delete(DeleteCluster),
    Get(GetCluster),
    Update(UpdateCluster),
    ExpandClusterDisk(ExpandClusterDisk),
}

impl CommandChoices {
    pub async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandChoices::List(params) => params.exec(cfg).await,
            CommandChoices::Create(params) => params.exec(cfg).await,
            CommandChoices::Delete(params) => params.exec(cfg).await,
            CommandChoices::Get(params) => params.exec(cfg).await,
            CommandChoices::Update(params) => params.exec(cfg).await,
            CommandChoices::ExpandClusterDisk(params) => params.exec(cfg).await,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "List clusters")]
pub struct ListClusters {
    #[structopt(long, help="The id of the organization the cluster is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the cluster is organized by",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
}

impl ListClusters {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::mesdb::paths::Clusters::new(sender)
            .list(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Create a cluster")]
pub struct CreateCluster {
    #[structopt(long, help="The id of the organization the cluster is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the cluster is organized by",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "A human-readable description of the cluster")]
    pub description: String,
    #[structopt(long, help = "Total disk capacity in Gigabytes (GB)")]
    pub disk_size_gb: i32,
    #[structopt(long)]
    pub disk_type: String,
    #[structopt(
        long,
        help = "Type of instance, based on its hardware. For example, it could be F1 for a micro or C4 for a large instance"
    )]
    pub instance_type: String,
    #[structopt(long, help = "The network id the cluster will be set on")]
    pub network_id: String,
    #[structopt(
        long,
        help = "The projection level of your database. Can be off, system or user"
    )]
    pub projection_level: String,
    #[structopt(long)]
    pub server_version: String,
    #[structopt(long, help = "Optional id of backup to restore")]
    pub source_backup_id: Option<String>,
    #[structopt(long, help = "Either single-node or three-node-multi-zone")]
    pub topology: String,
}

impl CreateCluster {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::mesdb::paths::Clusters::new(sender)
            .create(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::mesdb::CreateClusterRequest {
                    description: self.description.clone(),
                    disk_size_gb: self.disk_size_gb.clone(),
                    disk_type: self.disk_type.clone(),
                    instance_type: self.instance_type.clone(),
                    network_id: esc_api::NetworkId(self.network_id.clone()),
                    projection_level: esc_api::mesdb::ProjectionLevel::from_str(
                        &self.projection_level,
                    )?,
                    server_version: self.server_version.clone(),
                    source_backup_id: self.source_backup_id.clone(),
                    topology: esc_api::mesdb::Topology::from_str(&self.topology)?,
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Deletes a cluster")]
pub struct DeleteCluster {
    #[structopt(long, help="The id of the organization the cluster is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the cluster is organized by",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the cluster to delete")]
    pub cluster_id: String,
}

impl DeleteCluster {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::mesdb::paths::Clusters::new(sender)
            .delete(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::ClusterId(self.cluster_id.clone()),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Get a single cluster")]
pub struct GetCluster {
    #[structopt(long, help="The id of the organization the cluster is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the cluster is organized by",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the cluster to retrieve")]
    pub cluster_id: String,
}

impl GetCluster {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::mesdb::paths::Clusters::new(sender)
            .get(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::ClusterId(self.cluster_id.clone()),
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Get a single cluster")]
pub struct UpdateCluster {
    #[structopt(long, help="The id of the organization the cluster is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the cluster is organized by",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the cluster to update")]
    pub cluster_id: String,
    #[structopt(long, help = "A human-readable description of the cluster")]
    pub description: String,
}

impl UpdateCluster {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let render_in_json = cfg.render_in_json();

        let sender = cfg.create_request_sender();

        let result = esc_api::mesdb::paths::Clusters::new(sender)
            .update(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::ClusterId(self.cluster_id.clone()),
                esc_api::mesdb::UpdateClusterRequest {
                    description: self.description.clone(),
                },
            )
            .await?;

        crate::print_output(render_in_json, result)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "expands a cluster's disk")]
pub struct ExpandClusterDisk {
    #[structopt(long, help="The id of the organization the cluster is owned by",  parse(try_from_str = crate::parse_default_org_id), default_value = "")]
    pub organization_id: String,
    #[structopt(long, help="The id of the project the cluster is organized by",  parse(try_from_str = crate::parse_default_project_id), default_value = "")]
    pub project_id: String,
    #[structopt(long, help = "The id of the cluster")]
    pub cluster_id: String,
    #[structopt(long, help = "Total disk capacity in Gigabytes (GB)")]
    pub disk_size_gb: i32,
}

impl ExpandClusterDisk {
    async fn exec(&self, cfg: &crate::CliConfig) -> Result<(), Box<dyn std::error::Error>> {
        let sender = cfg.create_request_sender();

        esc_api::mesdb::paths::Clusters::new(sender)
            .expand_cluster_disk(
                esc_api::OrgId(self.organization_id.clone()),
                esc_api::ProjectId(self.project_id.clone()),
                esc_api::ClusterId(self.cluster_id.clone()),
                esc_api::mesdb::ExpandClusterDiskRequest {
                    disk_size_gb: self.disk_size_gb.clone(),
                },
            )
            .await?;

        Ok(())
    }
}
