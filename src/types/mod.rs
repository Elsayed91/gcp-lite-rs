//! GCP API type definitions.

pub mod accessapproval;
pub mod apikeys;
pub mod appengine;
pub mod bigquery;
pub mod cloudasset;
pub mod cloudbilling;
pub mod cloudkms;
pub mod cloudresourcemanager;
pub mod cloudscheduler;
pub mod compute;
pub mod container;
pub mod dlp;
pub mod dns;
pub mod essentialcontacts;
pub mod gkebackup;
pub mod iam;
pub mod logging;
pub mod monitoring;
pub mod osconfig;
pub mod recommender;
pub mod secretmanager;
pub mod service_usage;
pub mod sqladmin;
pub mod storage;

pub use accessapproval::{AccessApprovalSettings, EnrolledService};
pub use apikeys::{
    V2AndroidApplication, V2AndroidKeyRestrictions, V2ApiTarget, V2BrowserKeyRestrictions,
    V2IosKeyRestrictions, V2Key, V2ListKeysResponse, V2Restrictions, V2ServerKeyRestrictions,
};
pub use appengine::{Application, NetworkSettings, Service};
pub use bigquery::{
    Clustering, Dataset, DatasetList, DatasetReference, ErrorProto, ExternalDatasetReference,
    JobCancelResponse, JobConfiguration, JobConfigurationQuery, JobList, JobReference,
    JobStatistics, QueryRequest, QueryResponse, RangePartitioning, Table, TableCell,
    TableFieldSchema, TableList, TableReference, TableRow, TableSchema, TimePartitioning,
};
pub use cloudasset::{
    Asset, IamPolicySearchResult, ListAssetsResponse, Resource, ResourceSearchResult,
    SearchAllIamPoliciesResponse, SearchAllResourcesResponse,
};
pub use cloudbilling::ProjectBillingInfo;
pub use cloudkms::{
    CryptoKey, CryptoKeyVersion, CryptoKeyVersionTemplate, KeyRing, ListCryptoKeysResponse,
    ListKeyRingsResponse, ListLocationsResponse, Location,
};
pub use cloudresourcemanager::{
    GetIamPolicyRequest, GetPolicyOptions, IamBinding, IamCondition, IamPolicy,
    ListProjectsResponse, MoveProjectRequest, Project, ProjectState, ProjectsLro,
    SearchProjectsResponse, TestIamPermissionsRequest, TestIamPermissionsResponse,
    UndeleteProjectRequest,
};
pub use cloudscheduler::{
    AppEngineHttpTarget, AppEngineRouting, HttpMethod, HttpTarget, JobState, ListJobsResponse,
    OAuthToken, OidcToken, PauseJobRequest, PubsubTarget, ResumeJobRequest, RetryConfig,
    RunJobRequest, SchedulerEmpty,
};
pub use compute::{
    AccessConfig, Address, AddressList, AddressStatus, AttachedDisk, Backend, BackendService,
    BackendServiceList, BackendServiceLogConfig, ConnectionDraining, Disk, DiskEncryptionKey,
    DiskList, DiskStatus, DisksResizeRequest, Firewall, FirewallDirection, FirewallList,
    GuestOsFeature, Instance, InstanceList, InstanceStatus, InstancesSetMachineTypeRequest,
    InstancesSetServiceAccountRequest, Metadata, NetworkInterface, Router, RouterNat,
    RouterNatLogConfig, RouterNatSubnetworkToNat, Scheduling, Snapshot, SnapshotList,
    SnapshotStatus, SslPoliciesList, SslPolicy, Subnetwork, SubnetworkLogConfig, Tags,
};
pub use container::{Cluster, ClusterStatus, ContainerLro, ListClustersResponse};
pub use dlp::{
    DiscoveryConfig, DlpDataRiskLevel, DlpSensitivityScore,
    GooglePrivacyDlpV2ListDiscoveryConfigsResponse,
    GooglePrivacyDlpV2ListProjectDataProfilesResponse, ProjectDataProfile,
};
pub use dns::{
    DnsKeySpec, DnsOperation, ManagedZone, ManagedZoneCloudLoggingConfig, ManagedZoneDnsSecConfig,
    ManagedZonesListResponse, PoliciesListResponse, PoliciesPatchResponse, PolicyNetwork,
};
pub use essentialcontacts::{EssentialContact, EssentialContactsEmpty, ListContactsResponse};
pub use gkebackup::{
    BackupConfig, BackupPlan, BackupPlanState, BackupSchedule, GkeBackupLro,
    ListBackupPlansResponse, RetentionPolicy,
};
pub use iam::{
    CreateServiceAccountKeyRequest, CreateServiceAccountRequest, IamEmpty,
    ListServiceAccountKeysResponse, ListServiceAccountsResponse, ServiceAccountKey,
};
pub use logging::{
    BigQueryOptions, ListLogMetricsResponse, ListSinksResponse, LogMetric, LogSink, LoggingEmpty,
};
pub use monitoring::{
    AlertPolicy, Condition, Documentation, ListAlertPoliciesResponse,
    ListMetricDescriptorsResponse, ListMonitoredResourceDescriptorsResponse,
    ListNotificationChannelsResponse, ListTimeSeriesResponse, Metric, MonitoredResource,
    MonitoredResourceDescriptor, MonitoredResourceMetadata, MonitoringEmpty, NotificationChannel,
    Point, TimeInterval, TimeSeries, TypedValue,
};
pub use osconfig::{
    Inventory, InventoryOsInfo, ListInventoriesResponse, ListPatchDeploymentsResponse,
    PatchDeployment,
};
pub use recommender::{
    CostProjection, Impact, ImpactCategory, ListRecommendationsResponse, Money, OperationGroup,
    Recommendation, RecommendationContent, RecommendationInsightReference, RecommendationOperation,
    RecommendationPriority, RecommendationState, RecommendationStateInfo, ReliabilityProjection,
    SecurityProjection, SustainabilityProjection,
};
pub use secretmanager::{
    Automatic, CustomerManagedEncryption, Empty, ListSecretsResponse, Replica, Replication,
    Rotation, Secret, UserManaged,
};
pub use service_usage::{
    BatchEnableServicesRequest, CheckIfServiceHasUsage, DisableServiceRequest,
    EnableServiceRequest, ServiceState, ServiceStateEnum, ServiceUsageLro,
};
pub use sqladmin::{
    AclEntry, BackupConfiguration, BackupRetentionSettings, BackupRun, BinLogCoordinates,
    CloneContext, Database, DatabaseFlags, DatabaseInstance, DatabasesListResponse,
    DiskEncryptionConfiguration, DiskEncryptionStatus, ExportContext, FailoverContext,
    ImportContext, InstanceState, InstancesCloneRequest, InstancesExportRequest,
    InstancesFailoverRequest, InstancesImportRequest, InstancesListResponse,
    InstancesRotateServerCaRequest, IpConfiguration, IpMapping, LocationPreference,
    MaintenanceWindow, MySqlReplicaConfiguration, OperationError, OperationErrors,
    OperationsListResponse, ReplicaConfiguration, RotateServerCaContext, Settings, SqlBackendType,
    SqlInstanceType, User, UsersListResponse,
};
pub use storage::{
    Bucket, BucketAccessControl, Buckets, ComposeRequest, Object, ObjectAccessControl, Objects,
    RewriteResponse,
};
