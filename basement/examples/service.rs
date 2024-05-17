//! service

use basement::{
    message::{Arguments, ServiceMessageStream},
    status::{CurrentState, ServiceControlAccept, ServiceType, StatusHandle},
};
use futures::StreamExt;
use tracing::{info, trace};
use tracing_subscriber::{filter::LevelFilter, fmt, layer::SubscriberExt, prelude::*};

fn main() {
    let fmt = fmt::layer().with_target(false);
    tracing_subscriber::registry()
        .with(fmt)
        .with(LevelFilter::TRACE)
        .init();
    info!("Application service starting...");

    // Register a ServiceMain function
    basement::start_service_ctrl_dispatcher![
        // The Altronix ZDK device management service
        ("Altronix ZDK Device Service", svc_dev),
        // The Altronix ZDK update service
        ("Altronix ZDK Update Service", svc_update),
    ];
}

/// This service will manage I/O communication with Altronix LinQ devices. Including managing
/// firmware updates.
#[basement::service(name = "Altronix ZDK Device Service", mt = true)]
async fn svc_dev(mut hstatus: StatusHandle, mut stream: ServiceMessageStream, _arg: Arguments) {
    // Print some welcome logs and set service status
    info!("Starting Altronix ZDK Device Service...");
    hstatus
        .set_service_type(ServiceType::UserShareProcess)
        .set_current_state(CurrentState::ServiceRunning)
        .set_control_accept(ServiceControlAccept::all())
        .set_status()?;

    // Register a status handle and listen for incoming SCM messages
    info!("Register Altronix ZDK Device Service StatusHandle Ok");
    while let Some(msg) = stream.next().await {
        trace!(?msg, "Received a SCM msg");
    }

    // We received a stop or shutdown, end our program
    info!("Altronix ZDK Device Service Stopping");
    hstatus
        .set_current_state(CurrentState::ServiceStopped)
        .set_status()?;
    info!("Altronix ZDK Device Service Stopped");
    std::io::Result::Ok(())
}

/// This service will manage Host updates. NOTE that this service does not manage firmware updates
/// for devices. Firmware updates for devices are managed by the "Device Service"
#[basement::service(name = "Altronix ZDK Update Service")]
async fn svc_update(mut hstatus: StatusHandle, mut stream: ServiceMessageStream, _arg: Arguments) {
    // Print some welcome logs and set service status
    info!("Starting Altronix ZDK Update Service...");
    hstatus
        .set_service_type(ServiceType::UserShareProcess)
        .set_current_state(CurrentState::ServiceRunning)
        .set_control_accept(ServiceControlAccept::all())
        .set_status()?;

    // Register a status handle and listen for incoming SCM messages
    info!("Register Altronix ZDK Update Service StatusHandle Ok");
    while let Some(msg) = stream.next().await {
        trace!(?msg, "Received a SCM msg");
    }

    // We received a stop or shutdown, end our program
    info!("Altronix ZDK Update Service Stopping");
    hstatus
        .set_current_state(CurrentState::ServiceStopped)
        .set_status()?;
    info!("Altronix ZDK Device Service Stopped");
    std::io::Result::Ok(())
}
