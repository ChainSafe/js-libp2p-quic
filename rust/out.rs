use std::{
  net::{IpAddr, SocketAddr},
  sync::Arc,
};

use napi::bindgen_prelude::*;


#[doc = " User-level configuration"]
pub struct Config {
  #[doc = " Libp2p identity of the node, protobuf encoded."]
  pub private_key_proto: Uint8Array,
  #[doc = " Timeout for the initial handshake when establishing a connection."]
  #[doc = " The actual timeout is the minimum of this and the [`Config::max_idle_timeout`]."]
  pub handshake_timeout: u32,
  #[doc = " Maximum duration of inactivity in ms to accept before timing out the connection."]
  pub max_idle_timeout: u32,
  #[doc = " Period of inactivity before sending a keep-alive packet."]
  #[doc = " Must be set lower than the idle_timeout of both"]
  #[doc = " peers to be effective."]
  #[doc = ""]
  #[doc = " See [`quinn::TransportConfig::keep_alive_interval`] for more"]
  #[doc = " info."]
  pub keep_alive_interval: u32,
  #[doc = " Maximum number of incoming bidirectional streams that may be open"]
  #[doc = " concurrently by the remote peer."]
  pub max_concurrent_stream_limit: u32,
  #[doc = " Max unacknowledged data in bytes that may be sent on a single stream."]
  pub max_stream_data: u32,
  #[doc = " Max unacknowledged data in bytes that may be sent in total on all streams"]
  #[doc = " of a connection."]
  pub max_connection_data: u32,
}
impl napi::bindgen_prelude::TypeName for Config {
  fn type_name() -> &'static str {
    "Config"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::ToNapiValue for Config {
  unsafe fn to_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    val: Config,
  ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
    let env_wrapper = napi::bindgen_prelude::Env::from(env);
    let mut obj = env_wrapper.create_object()?;
    let Self {
      private_key_proto: private_key_proto_,
      handshake_timeout: handshake_timeout_,
      max_idle_timeout: max_idle_timeout_,
      keep_alive_interval: keep_alive_interval_,
      max_concurrent_stream_limit: max_concurrent_stream_limit_,
      max_stream_data: max_stream_data_,
      max_connection_data: max_connection_data_,
    } = val;
    obj.set("privateKeyProto", private_key_proto_)?;
    obj.set("handshakeTimeout", handshake_timeout_)?;
    obj.set("maxIdleTimeout", max_idle_timeout_)?;
    obj.set("keepAliveInterval", keep_alive_interval_)?;
    obj.set("maxConcurrentStreamLimit", max_concurrent_stream_limit_)?;
    obj.set("maxStreamData", max_stream_data_)?;
    obj.set("maxConnectionData", max_connection_data_)?;
    napi::bindgen_prelude::Object::to_napi_value(env, obj)
  }
}
impl napi::bindgen_prelude::FromNapiValue for Config {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    let env_wrapper = napi::bindgen_prelude::Env::from(env);
    let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
    let private_key_proto_: Uint8Array = obj
      .get("privateKeyProto")
      .map_err(|mut err| {
        err.reason = format!("{} on {}.{}", err.reason, "Config", "privateKeyProto");
        err
      })?
      .ok_or_else(|| {
        napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          format!("Missing field `{}`", "privateKeyProto"),
        )
      })?;
    let handshake_timeout_: u32 = obj
      .get("handshakeTimeout")
      .map_err(|mut err| {
        err.reason = format!("{} on {}.{}", err.reason, "Config", "handshakeTimeout");
        err
      })?
      .ok_or_else(|| {
        napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          format!("Missing field `{}`", "handshakeTimeout"),
        )
      })?;
    let max_idle_timeout_: u32 = obj
      .get("maxIdleTimeout")
      .map_err(|mut err| {
        err.reason = format!("{} on {}.{}", err.reason, "Config", "maxIdleTimeout");
        err
      })?
      .ok_or_else(|| {
        napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          format!("Missing field `{}`", "maxIdleTimeout"),
        )
      })?;
    let keep_alive_interval_: u32 = obj
      .get("keepAliveInterval")
      .map_err(|mut err| {
        err.reason = format!("{} on {}.{}", err.reason, "Config", "keepAliveInterval");
        err
      })?
      .ok_or_else(|| {
        napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          format!("Missing field `{}`", "keepAliveInterval"),
        )
      })?;
    let max_concurrent_stream_limit_: u32 = obj
      .get("maxConcurrentStreamLimit")
      .map_err(|mut err| {
        err.reason = format!(
          "{} on {}.{}",
          err.reason, "Config", "maxConcurrentStreamLimit"
        );
        err
      })?
      .ok_or_else(|| {
        napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          format!("Missing field `{}`", "maxConcurrentStreamLimit"),
        )
      })?;
    let max_stream_data_: u32 = obj
      .get("maxStreamData")
      .map_err(|mut err| {
        err.reason = format!("{} on {}.{}", err.reason, "Config", "maxStreamData");
        err
      })?
      .ok_or_else(|| {
        napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          format!("Missing field `{}`", "maxStreamData"),
        )
      })?;
    let max_connection_data_: u32 = obj
      .get("maxConnectionData")
      .map_err(|mut err| {
        err.reason = format!("{} on {}.{}", err.reason, "Config", "maxConnectionData");
        err
      })?
      .ok_or_else(|| {
        napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          format!("Missing field `{}`", "maxConnectionData"),
        )
      })?;
    let val = Self {
      private_key_proto: private_key_proto_,
      handshake_timeout: handshake_timeout_,
      max_idle_timeout: max_idle_timeout_,
      keep_alive_interval: keep_alive_interval_,
      max_concurrent_stream_limit: max_concurrent_stream_limit_,
      max_stream_data: max_stream_data_,
      max_connection_data: max_connection_data_,
    };
    Ok(val)
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for Config {}
#[doc = " Configuration used by the QUIC library"]
pub struct QuinnConfig {
  pub(crate) client_config: quinn::ClientConfig,
  pub(crate) server_config: quinn::ServerConfig,
  pub(crate) endpoint_config: quinn::EndpointConfig,
}
impl napi::bindgen_prelude::TypeName for QuinnConfig {
  fn type_name() -> &'static str {
    "QuinnConfig"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Function
  }
}
impl napi::bindgen_prelude::TypeName for &QuinnConfig {
  fn type_name() -> &'static str {
    "QuinnConfig"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::TypeName for &mut QuinnConfig {
  fn type_name() -> &'static str {
    "QuinnConfig"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::ToNapiValue for QuinnConfig {
  unsafe fn to_napi_value(
    env: napi::sys::napi_env,
    val: QuinnConfig,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    if let Some(ctor_ref) = napi::__private::get_class_constructor("QuinnConfig\0") {
      let wrapped_value = Box::into_raw(Box::new(val));
      let instance_value = QuinnConfig::new_instance(env, wrapped_value.cast(), ctor_ref)?;
      Ok(instance_value)
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!(
          "Failed to get constructor of class `{}` in `ToNapiValue`",
          "QuinnConfig"
        ),
      ))
    }
  }
}
impl napi::bindgen_prelude::ObjectFinalize for QuinnConfig {}
impl QuinnConfig {
  pub fn instance_of<V: napi::NapiRaw>(env: napi::Env, value: V) -> napi::Result<bool> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("QuinnConfig\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        unsafe { napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor) },
        "Failed to get constructor reference of class `{}`",
        "QuinnConfig\0"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        unsafe { napi::sys::napi_instanceof(env.raw(), value.raw(), ctor, &mut is_instance_of) },
        "Failed to run instanceof for class `{}`",
        "QuinnConfig\0"
      )?;
      Ok(is_instance_of)
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to get constructor of class `{}`", "QuinnConfig\0"),
      ))
    }
  }
}
impl QuinnConfig {
  pub fn into_reference(
    val: QuinnConfig,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::Reference<QuinnConfig>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("QuinnConfig\0") {
      unsafe {
        let wrapped_value = Box::into_raw(Box::new(val));
        let instance_value = QuinnConfig::new_instance(env.raw(), wrapped_value.cast(), ctor_ref)?;
        {
          let env = env.raw();
        }
        napi::bindgen_prelude::Reference::<QuinnConfig>::from_value_ptr(
          wrapped_value.cast(),
          env.raw(),
        )
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "QuinnConfig"),
      ))
    }
  }
  pub fn into_instance(
    self,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::ClassInstance<QuinnConfig>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("QuinnConfig\0") {
      unsafe {
        let wrapped_value = Box::leak(Box::new(self));
        let instance_value = QuinnConfig::new_instance(
          env.raw(),
          wrapped_value as *mut _ as *mut std::ffi::c_void,
          ctor_ref,
        )?;
        Ok(napi::bindgen_prelude::ClassInstance::<QuinnConfig>::new(
          instance_value,
          wrapped_value,
        ))
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "QuinnConfig"),
      ))
    }
  }
  unsafe fn new_instance(
    env: napi::sys::napi_env,
    wrapped_value: *mut std::ffi::c_void,
    ctor_ref: napi::sys::napi_ref,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut ctor = std::ptr::null_mut();
    napi::check_status!(
      napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
      "Failed to get constructor reference of class `{}`",
      "QuinnConfig"
    )?;
    let mut result = std::ptr::null_mut();
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
    napi::check_status!(
      napi::sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result),
      "Failed to construct class `{}`",
      "QuinnConfig"
    )?;
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
    let mut object_ref = std::ptr::null_mut();
    let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
    let finalize_callbacks_ptr = std::rc::Rc::into_raw(std::rc::Rc::new(std::cell::Cell::new(
      Box::into_raw(initial_finalize),
    )));
    napi::check_status!(
      napi::sys::napi_wrap(
        env,
        result,
        wrapped_value,
        Some(napi::bindgen_prelude::raw_finalize_unchecked::<QuinnConfig>),
        std::ptr::null_mut(),
        &mut object_ref,
      ),
      "Failed to wrap native object of class `{}`",
      "QuinnConfig"
    )?;
    napi::bindgen_prelude::Reference::<QuinnConfig>::add_ref(
      env,
      wrapped_value,
      (wrapped_value, object_ref, finalize_callbacks_ptr),
    );
    Ok(result)
  }
}
impl napi::bindgen_prelude::FromNapiRef for QuinnConfig {
  unsafe fn from_napi_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "QuinnConfig",
    )?;
    Ok(&*(wrapped_val as *const QuinnConfig))
  }
}
impl napi::bindgen_prelude::FromNapiMutRef for QuinnConfig {
  unsafe fn from_napi_mut_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static mut Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "QuinnConfig",
    )?;
    Ok(&mut *(wrapped_val as *mut QuinnConfig))
  }
}
impl napi::bindgen_prelude::FromNapiValue for &QuinnConfig {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::FromNapiValue for &mut QuinnConfig {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &QuinnConfig {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("QuinnConfig\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "QuinnConfig"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "QuinnConfig"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "QuinnConfig"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "QuinnConfig"),
      ))
    }
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &mut QuinnConfig {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("QuinnConfig\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "QuinnConfig"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "QuinnConfig"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "QuinnConfig"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "QuinnConfig"),
      ))
    }
  }
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
mod __napi_helper__QuinnConfig {
  use super::*;
  use std::ptr;
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__QuinnConfig_struct_1() {
    napi::__private::register_class("QuinnConfig", None, "QuinnConfig\0", vec![]);
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__QuinnConfig_struct_1() {
    napi::__private::register_class("QuinnConfig", None, "QuinnConfig\0", vec![]);
  }
}
impl QuinnConfig {
  pub fn new(config: Config) -> Result<Self> {
    Ok(QuinnConfig::try_from(config)?)
  }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod __napi_impl_helper__QuinnConfig__0 {
  use super::*;
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__new(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      if napi::__private::___CALL_FROM_FACTORY
        .with(|inner| inner.load(std::sync::atomic::Ordering::Relaxed))
      {
        return std::ptr::null_mut();
      }
      napi::bindgen_prelude::CallbackInfo::<1usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let arg0 = {
            <Config as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
              env,
              cb.get_arg(0usize),
            )?
          };
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { QuinnConfig::new(arg0) };
            match _ret {
              Ok(value) => cb.construct::<false, QuinnConfig>("constructor", value),
              Err(err) => {
                napi::bindgen_prelude::JsError::from(err).throw_into(env);
                Ok(std::ptr::null_mut())
              }
            }
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__QuinnConfig_impl_3() {
    napi::__private::register_class(
      "QuinnConfig",
      None,
      "QuinnConfig\0",
      vec![napi::bindgen_prelude::Property::new("constructor")
        .unwrap()
        .with_property_attributes(
          napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
        )
        .with_ctor(__napi__new)],
    );
  }
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__QuinnConfig_impl_3() {
    napi::__private::register_class(
      "QuinnConfig",
      None,
      "QuinnConfig\0",
      vec![napi::bindgen_prelude::Property::new("constructor")
        .unwrap()
        .with_property_attributes(
          napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
        )
        .with_ctor(__napi__new)],
    );
  }
}
pub struct EndpointStats {
  #[doc = " Cummulative number of Quic handshakes accepted by this Endpoint"]
  pub accepted_handshakes: u32,
  #[doc = " Cummulative number of Quic handshakees sent from this Endpoint"]
  pub outgoing_handshakes: u32,
  #[doc = " Cummulative number of Quic handshakes refused on this Endpoint"]
  pub refused_handshakes: u32,
  #[doc = " Cummulative number of Quic handshakes ignored on this Endpoint"]
  pub ignored_handshakes: u32,
}
impl napi::bindgen_prelude::TypeName for EndpointStats {
  fn type_name() -> &'static str {
    "EndpointStats"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Function
  }
}
impl napi::bindgen_prelude::TypeName for &EndpointStats {
  fn type_name() -> &'static str {
    "EndpointStats"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::TypeName for &mut EndpointStats {
  fn type_name() -> &'static str {
    "EndpointStats"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::ToNapiValue for EndpointStats {
  unsafe fn to_napi_value(
    env: napi::sys::napi_env,
    val: EndpointStats,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    if let Some(ctor_ref) = napi::__private::get_class_constructor("EndpointStats\0") {
      let wrapped_value = Box::into_raw(Box::new(val));
      let instance_value = EndpointStats::new_instance(env, wrapped_value.cast(), ctor_ref)?;
      Ok(instance_value)
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!(
          "Failed to get constructor of class `{}` in `ToNapiValue`",
          "EndpointStats"
        ),
      ))
    }
  }
}
impl napi::bindgen_prelude::ObjectFinalize for EndpointStats {}
impl EndpointStats {
  pub fn instance_of<V: napi::NapiRaw>(env: napi::Env, value: V) -> napi::Result<bool> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("EndpointStats\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        unsafe { napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor) },
        "Failed to get constructor reference of class `{}`",
        "EndpointStats\0"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        unsafe { napi::sys::napi_instanceof(env.raw(), value.raw(), ctor, &mut is_instance_of) },
        "Failed to run instanceof for class `{}`",
        "EndpointStats\0"
      )?;
      Ok(is_instance_of)
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to get constructor of class `{}`", "EndpointStats\0"),
      ))
    }
  }
}
impl EndpointStats {
  pub fn into_reference(
    val: EndpointStats,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::Reference<EndpointStats>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("EndpointStats\0") {
      unsafe {
        let wrapped_value = Box::into_raw(Box::new(val));
        let instance_value =
          EndpointStats::new_instance(env.raw(), wrapped_value.cast(), ctor_ref)?;
        {
          let env = env.raw();
        }
        napi::bindgen_prelude::Reference::<EndpointStats>::from_value_ptr(
          wrapped_value.cast(),
          env.raw(),
        )
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "EndpointStats"),
      ))
    }
  }
  pub fn into_instance(
    self,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::ClassInstance<EndpointStats>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("EndpointStats\0") {
      unsafe {
        let wrapped_value = Box::leak(Box::new(self));
        let instance_value = EndpointStats::new_instance(
          env.raw(),
          wrapped_value as *mut _ as *mut std::ffi::c_void,
          ctor_ref,
        )?;
        Ok(napi::bindgen_prelude::ClassInstance::<EndpointStats>::new(
          instance_value,
          wrapped_value,
        ))
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "EndpointStats"),
      ))
    }
  }
  unsafe fn new_instance(
    env: napi::sys::napi_env,
    wrapped_value: *mut std::ffi::c_void,
    ctor_ref: napi::sys::napi_ref,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut ctor = std::ptr::null_mut();
    napi::check_status!(
      napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
      "Failed to get constructor reference of class `{}`",
      "EndpointStats"
    )?;
    let mut result = std::ptr::null_mut();
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
    napi::check_status!(
      napi::sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result),
      "Failed to construct class `{}`",
      "EndpointStats"
    )?;
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
    let mut object_ref = std::ptr::null_mut();
    let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
    let finalize_callbacks_ptr = std::rc::Rc::into_raw(std::rc::Rc::new(std::cell::Cell::new(
      Box::into_raw(initial_finalize),
    )));
    napi::check_status!(
      napi::sys::napi_wrap(
        env,
        result,
        wrapped_value,
        Some(napi::bindgen_prelude::raw_finalize_unchecked::<EndpointStats>),
        std::ptr::null_mut(),
        &mut object_ref,
      ),
      "Failed to wrap native object of class `{}`",
      "EndpointStats"
    )?;
    napi::bindgen_prelude::Reference::<EndpointStats>::add_ref(
      env,
      wrapped_value,
      (wrapped_value, object_ref, finalize_callbacks_ptr),
    );
    Ok(result)
  }
}
impl napi::bindgen_prelude::FromNapiRef for EndpointStats {
  unsafe fn from_napi_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "EndpointStats",
    )?;
    Ok(&*(wrapped_val as *const EndpointStats))
  }
}
impl napi::bindgen_prelude::FromNapiMutRef for EndpointStats {
  unsafe fn from_napi_mut_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static mut Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "EndpointStats",
    )?;
    Ok(&mut *(wrapped_val as *mut EndpointStats))
  }
}
impl napi::bindgen_prelude::FromNapiValue for &EndpointStats {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::FromNapiValue for &mut EndpointStats {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &EndpointStats {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("EndpointStats\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "EndpointStats"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "EndpointStats"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "EndpointStats"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "EndpointStats"),
      ))
    }
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &mut EndpointStats {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("EndpointStats\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "EndpointStats"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "EndpointStats"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "EndpointStats"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "EndpointStats"),
      ))
    }
  }
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
mod __napi_helper__EndpointStats {
  use super::*;
  use std::ptr;
  extern "C" fn get_accepted_handshakes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<EndpointStats>() })
      .and_then(|obj| {
        let val = obj.accepted_handshakes.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_accepted_handshakes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<EndpointStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.accepted_handshakes = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_ignored_handshakes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<EndpointStats>() })
      .and_then(|obj| {
        let val = obj.ignored_handshakes.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_ignored_handshakes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<EndpointStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.ignored_handshakes = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_outgoing_handshakes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<EndpointStats>() })
      .and_then(|obj| {
        let val = obj.outgoing_handshakes.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_outgoing_handshakes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<EndpointStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.outgoing_handshakes = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_refused_handshakes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<EndpointStats>() })
      .and_then(|obj| {
        let val = obj.refused_handshakes.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_refused_handshakes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<EndpointStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.refused_handshakes = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__EndpointStats_struct_4() {
    napi::__private::register_class(
      "EndpointStats",
      None,
      "EndpointStats\0",
      vec![
        napi::bindgen_prelude::Property::new("acceptedHandshakes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_accepted_handshakes)
          .with_setter(set_accepted_handshakes),
        napi::bindgen_prelude::Property::new("outgoingHandshakes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_outgoing_handshakes)
          .with_setter(set_outgoing_handshakes),
        napi::bindgen_prelude::Property::new("refusedHandshakes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_refused_handshakes)
          .with_setter(set_refused_handshakes),
        napi::bindgen_prelude::Property::new("ignoredHandshakes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_ignored_handshakes)
          .with_setter(set_ignored_handshakes),
      ],
    );
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__EndpointStats_struct_4() {
    napi::__private::register_class(
      "EndpointStats",
      None,
      "EndpointStats\0",
      vec![
        napi::bindgen_prelude::Property::new("acceptedHandshakes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_accepted_handshakes)
          .with_setter(set_accepted_handshakes),
        napi::bindgen_prelude::Property::new("outgoingHandshakes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_outgoing_handshakes)
          .with_setter(set_outgoing_handshakes),
        napi::bindgen_prelude::Property::new("refusedHandshakes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_refused_handshakes)
          .with_setter(set_refused_handshakes),
        napi::bindgen_prelude::Property::new("ignoredHandshakes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_ignored_handshakes)
          .with_setter(set_ignored_handshakes),
      ],
    );
  }
}
pub struct ConnectionStats {
  #[doc = " Statistics about UDP datagrams transmitted on a connection"]
  #[doc = ""]
  #[doc = " The amount of UDP datagrams observed"]
  pub udp_tx_datagrams: u32,
  #[doc = " Statistics about UDP datagrams transmitted on a connection"]
  #[doc = ""]
  #[doc = " The total amount of bytes which have been transferred inside UDP datagrams"]
  pub udp_tx_bytes: u32,
  #[doc = " Statistics about UDP datagrams transmitted on a connection"]
  #[doc = ""]
  #[doc = " The amount of I/O operations executed"]
  #[doc = ""]
  #[doc = " Can be less than `datagrams` when GSO, GRO, and/or batched system calls are in use."]
  pub udp_tx_ios: u32,
  #[doc = " Statistics about UDP datagrams received on a connection"]
  #[doc = ""]
  #[doc = " The amount of UDP datagrams observed"]
  pub udp_rx_datagrams: u32,
  #[doc = " Statistics about UDP datagrams received on a connection"]
  #[doc = ""]
  #[doc = " The total amount of bytes which have been transferred inside UDP datagrams"]
  pub udp_rx_bytes: u32,
  #[doc = " Statistics about UDP datagrams received on a connection"]
  #[doc = ""]
  #[doc = " The amount of I/O operations executed"]
  #[doc = ""]
  #[doc = " Can be less than `datagrams` when GSO, GRO, and/or batched system calls are in use."]
  pub udp_rx_ios: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_acks: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_ack_frequency: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_crypto: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_connection_close: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_data_blocked: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_datagram: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_handshake_done: u8,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_immediate_ack: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_max_data: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_max_stream_data: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_max_streams_bidi: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_max_streams_uni: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_new_connection_id: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_new_token: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_path_challenge: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_path_response: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_ping: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_reset_stream: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_retire_connection_id: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_stream_data_blocked: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_streams_blocked_bidi: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_streams_blocked_uni: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_stop_sending: u32,
  #[doc = " Statistics about frames transmitted on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_tx_stream: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_acks: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_ack_frequency: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_crypto: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_connection_close: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_data_blocked: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_datagram: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_handshake_done: u8,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_immediate_ack: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_max_data: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_max_stream_data: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_max_streams_bidi: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_max_streams_uni: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_new_connection_id: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_new_token: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_path_challenge: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_path_response: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_ping: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_reset_stream: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_retire_connection_id: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_stream_data_blocked: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_streams_blocked_bidi: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_streams_blocked_uni: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_stop_sending: u32,
  #[doc = " Statistics about frames received on a connection"]
  #[doc = " Number of frames transmitted of this frame type"]
  pub frame_rx_stream: u32,
  #[doc = " Current best estimate of this connection\'s latency (round-trip-time)"]
  pub path_rtt: u32,
  #[doc = " Current congestion window of the connection"]
  pub path_cwnd: u32,
  #[doc = " Congestion events on the connection"]
  pub path_congestion_events: u32,
  #[doc = " The amount of packets lost on this path"]
  pub path_lost_packets: u32,
  #[doc = " The amount of bytes lost on this path"]
  pub path_lost_bytes: u32,
  #[doc = " The amount of packets sent on this path"]
  pub path_sent_packets: u32,
  #[doc = " The amount of PLPMTUD probe packets sent on this path (also counted by `sent_packets`)"]
  pub path_sent_plpmtud_probes: u32,
  #[doc = " The amount of PLPMTUD probe packets lost on this path (ignored by `lost_packets` and"]
  #[doc = " `lost_bytes`)"]
  pub path_lost_plpmtud_probes: u32,
  #[doc = " The number of times a black hole was detected in the path"]
  pub path_black_holes_detected: u32,
}
impl napi::bindgen_prelude::TypeName for ConnectionStats {
  fn type_name() -> &'static str {
    "ConnectionStats"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Function
  }
}
impl napi::bindgen_prelude::TypeName for &ConnectionStats {
  fn type_name() -> &'static str {
    "ConnectionStats"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::TypeName for &mut ConnectionStats {
  fn type_name() -> &'static str {
    "ConnectionStats"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::ToNapiValue for ConnectionStats {
  unsafe fn to_napi_value(
    env: napi::sys::napi_env,
    val: ConnectionStats,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    if let Some(ctor_ref) = napi::__private::get_class_constructor("ConnectionStats\0") {
      let wrapped_value = Box::into_raw(Box::new(val));
      let instance_value = ConnectionStats::new_instance(env, wrapped_value.cast(), ctor_ref)?;
      Ok(instance_value)
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!(
          "Failed to get constructor of class `{}` in `ToNapiValue`",
          "ConnectionStats"
        ),
      ))
    }
  }
}
impl napi::bindgen_prelude::ObjectFinalize for ConnectionStats {}
impl ConnectionStats {
  pub fn instance_of<V: napi::NapiRaw>(env: napi::Env, value: V) -> napi::Result<bool> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("ConnectionStats\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        unsafe { napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor) },
        "Failed to get constructor reference of class `{}`",
        "ConnectionStats\0"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        unsafe { napi::sys::napi_instanceof(env.raw(), value.raw(), ctor, &mut is_instance_of) },
        "Failed to run instanceof for class `{}`",
        "ConnectionStats\0"
      )?;
      Ok(is_instance_of)
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!(
          "Failed to get constructor of class `{}`",
          "ConnectionStats\0"
        ),
      ))
    }
  }
}
impl ConnectionStats {
  pub fn into_reference(
    val: ConnectionStats,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::Reference<ConnectionStats>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("ConnectionStats\0") {
      unsafe {
        let wrapped_value = Box::into_raw(Box::new(val));
        let instance_value =
          ConnectionStats::new_instance(env.raw(), wrapped_value.cast(), ctor_ref)?;
        {
          let env = env.raw();
        }
        napi::bindgen_prelude::Reference::<ConnectionStats>::from_value_ptr(
          wrapped_value.cast(),
          env.raw(),
        )
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "ConnectionStats"),
      ))
    }
  }
  pub fn into_instance(
    self,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::ClassInstance<ConnectionStats>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("ConnectionStats\0") {
      unsafe {
        let wrapped_value = Box::leak(Box::new(self));
        let instance_value = ConnectionStats::new_instance(
          env.raw(),
          wrapped_value as *mut _ as *mut std::ffi::c_void,
          ctor_ref,
        )?;
        Ok(
          napi::bindgen_prelude::ClassInstance::<ConnectionStats>::new(
            instance_value,
            wrapped_value,
          ),
        )
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "ConnectionStats"),
      ))
    }
  }
  unsafe fn new_instance(
    env: napi::sys::napi_env,
    wrapped_value: *mut std::ffi::c_void,
    ctor_ref: napi::sys::napi_ref,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut ctor = std::ptr::null_mut();
    napi::check_status!(
      napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
      "Failed to get constructor reference of class `{}`",
      "ConnectionStats"
    )?;
    let mut result = std::ptr::null_mut();
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
    napi::check_status!(
      napi::sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result),
      "Failed to construct class `{}`",
      "ConnectionStats"
    )?;
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
    let mut object_ref = std::ptr::null_mut();
    let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
    let finalize_callbacks_ptr = std::rc::Rc::into_raw(std::rc::Rc::new(std::cell::Cell::new(
      Box::into_raw(initial_finalize),
    )));
    napi::check_status!(
      napi::sys::napi_wrap(
        env,
        result,
        wrapped_value,
        Some(napi::bindgen_prelude::raw_finalize_unchecked::<ConnectionStats>),
        std::ptr::null_mut(),
        &mut object_ref,
      ),
      "Failed to wrap native object of class `{}`",
      "ConnectionStats"
    )?;
    napi::bindgen_prelude::Reference::<ConnectionStats>::add_ref(
      env,
      wrapped_value,
      (wrapped_value, object_ref, finalize_callbacks_ptr),
    );
    Ok(result)
  }
}
impl napi::bindgen_prelude::FromNapiRef for ConnectionStats {
  unsafe fn from_napi_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "ConnectionStats",
    )?;
    Ok(&*(wrapped_val as *const ConnectionStats))
  }
}
impl napi::bindgen_prelude::FromNapiMutRef for ConnectionStats {
  unsafe fn from_napi_mut_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static mut Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "ConnectionStats",
    )?;
    Ok(&mut *(wrapped_val as *mut ConnectionStats))
  }
}
impl napi::bindgen_prelude::FromNapiValue for &ConnectionStats {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::FromNapiValue for &mut ConnectionStats {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &ConnectionStats {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("ConnectionStats\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "ConnectionStats"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "ConnectionStats"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "ConnectionStats"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "ConnectionStats"),
      ))
    }
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &mut ConnectionStats {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("ConnectionStats\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "ConnectionStats"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "ConnectionStats"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "ConnectionStats"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "ConnectionStats"),
      ))
    }
  }
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
mod __napi_helper__ConnectionStats {
  use super::*;
  use std::ptr;
  extern "C" fn get_frame_rx_ack_frequency(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_ack_frequency.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_ack_frequency(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_ack_frequency = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_acks(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_acks.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_acks(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_acks = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_connection_close(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_connection_close.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_connection_close(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_connection_close = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_crypto(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_crypto.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_crypto(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_crypto = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_data_blocked(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_data_blocked.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_data_blocked(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_data_blocked = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_datagram(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_datagram.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_datagram(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_datagram = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_handshake_done(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_handshake_done.to_owned();
        unsafe { <u8 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_handshake_done(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u8 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_handshake_done = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_immediate_ack(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_immediate_ack.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_immediate_ack(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_immediate_ack = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_max_data(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_max_data.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_max_data(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_max_data = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_max_stream_data(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_max_stream_data.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_max_stream_data(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_max_stream_data = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_max_streams_bidi(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_max_streams_bidi.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_max_streams_bidi(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_max_streams_bidi = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_max_streams_uni(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_max_streams_uni.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_max_streams_uni(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_max_streams_uni = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_new_connection_id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_new_connection_id.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_new_connection_id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_new_connection_id = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_new_token(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_new_token.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_new_token(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_new_token = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_path_challenge(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_path_challenge.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_path_challenge(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_path_challenge = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_path_response(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_path_response.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_path_response(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_path_response = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_ping(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_ping.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_ping(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_ping = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_reset_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_reset_stream.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_reset_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_reset_stream = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_retire_connection_id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_retire_connection_id.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_retire_connection_id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_retire_connection_id = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_stop_sending(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_stop_sending.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_stop_sending(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_stop_sending = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_stream.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_stream = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_stream_data_blocked(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_stream_data_blocked.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_stream_data_blocked(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_stream_data_blocked = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_streams_blocked_bidi(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_streams_blocked_bidi.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_streams_blocked_bidi(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_streams_blocked_bidi = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_rx_streams_blocked_uni(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_rx_streams_blocked_uni.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_rx_streams_blocked_uni(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_rx_streams_blocked_uni = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_ack_frequency(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_ack_frequency.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_ack_frequency(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_ack_frequency = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_acks(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_acks.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_acks(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_acks = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_connection_close(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_connection_close.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_connection_close(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_connection_close = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_crypto(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_crypto.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_crypto(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_crypto = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_data_blocked(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_data_blocked.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_data_blocked(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_data_blocked = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_datagram(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_datagram.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_datagram(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_datagram = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_handshake_done(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_handshake_done.to_owned();
        unsafe { <u8 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_handshake_done(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u8 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_handshake_done = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_immediate_ack(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_immediate_ack.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_immediate_ack(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_immediate_ack = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_max_data(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_max_data.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_max_data(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_max_data = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_max_stream_data(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_max_stream_data.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_max_stream_data(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_max_stream_data = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_max_streams_bidi(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_max_streams_bidi.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_max_streams_bidi(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_max_streams_bidi = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_max_streams_uni(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_max_streams_uni.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_max_streams_uni(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_max_streams_uni = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_new_connection_id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_new_connection_id.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_new_connection_id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_new_connection_id = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_new_token(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_new_token.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_new_token(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_new_token = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_path_challenge(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_path_challenge.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_path_challenge(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_path_challenge = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_path_response(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_path_response.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_path_response(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_path_response = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_ping(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_ping.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_ping(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_ping = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_reset_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_reset_stream.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_reset_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_reset_stream = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_retire_connection_id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_retire_connection_id.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_retire_connection_id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_retire_connection_id = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_stop_sending(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_stop_sending.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_stop_sending(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_stop_sending = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_stream.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_stream = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_stream_data_blocked(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_stream_data_blocked.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_stream_data_blocked(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_stream_data_blocked = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_streams_blocked_bidi(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_streams_blocked_bidi.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_streams_blocked_bidi(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_streams_blocked_bidi = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_frame_tx_streams_blocked_uni(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.frame_tx_streams_blocked_uni.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_frame_tx_streams_blocked_uni(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.frame_tx_streams_blocked_uni = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_path_black_holes_detected(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.path_black_holes_detected.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_path_black_holes_detected(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.path_black_holes_detected = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_path_congestion_events(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.path_congestion_events.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_path_congestion_events(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.path_congestion_events = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_path_cwnd(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.path_cwnd.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_path_cwnd(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.path_cwnd = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_path_lost_bytes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.path_lost_bytes.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_path_lost_bytes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.path_lost_bytes = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_path_lost_packets(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.path_lost_packets.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_path_lost_packets(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.path_lost_packets = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_path_lost_plpmtud_probes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.path_lost_plpmtud_probes.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_path_lost_plpmtud_probes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.path_lost_plpmtud_probes = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_path_rtt(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.path_rtt.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_path_rtt(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.path_rtt = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_path_sent_packets(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.path_sent_packets.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_path_sent_packets(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.path_sent_packets = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_path_sent_plpmtud_probes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.path_sent_plpmtud_probes.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_path_sent_plpmtud_probes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.path_sent_plpmtud_probes = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_udp_rx_bytes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.udp_rx_bytes.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_udp_rx_bytes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.udp_rx_bytes = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_udp_rx_datagrams(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.udp_rx_datagrams.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_udp_rx_datagrams(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.udp_rx_datagrams = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_udp_rx_ios(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.udp_rx_ios.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_udp_rx_ios(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.udp_rx_ios = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_udp_tx_bytes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.udp_tx_bytes.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_udp_tx_bytes(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.udp_tx_bytes = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_udp_tx_datagrams(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.udp_tx_datagrams.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_udp_tx_datagrams(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.udp_tx_datagrams = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn get_udp_tx_ios(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<0>::new(env, cb, Some(0), false)
      .and_then(|mut cb| unsafe { cb.unwrap_borrow_mut::<ConnectionStats>() })
      .and_then(|obj| {
        let val = obj.udp_tx_ios.to_owned();
        unsafe { <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, val) }
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  extern "C" fn set_udp_tx_ios(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    napi::bindgen_prelude::CallbackInfo::<1>::new(env, cb, Some(1), false)
      .and_then(|mut cb_info| unsafe {
        cb_info
          .unwrap_borrow_mut::<ConnectionStats>()
          .and_then(|obj| {
            <u32 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb_info.get_arg(0))
              .and_then(move |val| {
                obj.udp_tx_ios = val;
                <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
              })
          })
      })
      .unwrap_or_else(|e| {
        unsafe { napi::bindgen_prelude::JsError::from(e).throw_into(env) };
        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
      })
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__ConnectionStats_struct_5() {
    napi::__private::register_class(
      "ConnectionStats",
      None,
      "ConnectionStats\0",
      vec![
        napi::bindgen_prelude::Property::new("udpTxDatagrams")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_tx_datagrams)
          .with_setter(set_udp_tx_datagrams),
        napi::bindgen_prelude::Property::new("udpTxBytes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_tx_bytes)
          .with_setter(set_udp_tx_bytes),
        napi::bindgen_prelude::Property::new("udpTxIos")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_tx_ios)
          .with_setter(set_udp_tx_ios),
        napi::bindgen_prelude::Property::new("udpRxDatagrams")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_rx_datagrams)
          .with_setter(set_udp_rx_datagrams),
        napi::bindgen_prelude::Property::new("udpRxBytes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_rx_bytes)
          .with_setter(set_udp_rx_bytes),
        napi::bindgen_prelude::Property::new("udpRxIos")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_rx_ios)
          .with_setter(set_udp_rx_ios),
        napi::bindgen_prelude::Property::new("frameTxAcks")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_acks)
          .with_setter(set_frame_tx_acks),
        napi::bindgen_prelude::Property::new("frameTxAckFrequency")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_ack_frequency)
          .with_setter(set_frame_tx_ack_frequency),
        napi::bindgen_prelude::Property::new("frameTxCrypto")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_crypto)
          .with_setter(set_frame_tx_crypto),
        napi::bindgen_prelude::Property::new("frameTxConnectionClose")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_connection_close)
          .with_setter(set_frame_tx_connection_close),
        napi::bindgen_prelude::Property::new("frameTxDataBlocked")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_data_blocked)
          .with_setter(set_frame_tx_data_blocked),
        napi::bindgen_prelude::Property::new("frameTxDatagram")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_datagram)
          .with_setter(set_frame_tx_datagram),
        napi::bindgen_prelude::Property::new("frameTxHandshakeDone")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_handshake_done)
          .with_setter(set_frame_tx_handshake_done),
        napi::bindgen_prelude::Property::new("frameTxImmediateAck")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_immediate_ack)
          .with_setter(set_frame_tx_immediate_ack),
        napi::bindgen_prelude::Property::new("frameTxMaxData")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_max_data)
          .with_setter(set_frame_tx_max_data),
        napi::bindgen_prelude::Property::new("frameTxMaxStreamData")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_max_stream_data)
          .with_setter(set_frame_tx_max_stream_data),
        napi::bindgen_prelude::Property::new("frameTxMaxStreamsBidi")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_max_streams_bidi)
          .with_setter(set_frame_tx_max_streams_bidi),
        napi::bindgen_prelude::Property::new("frameTxMaxStreamsUni")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_max_streams_uni)
          .with_setter(set_frame_tx_max_streams_uni),
        napi::bindgen_prelude::Property::new("frameTxNewConnectionId")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_new_connection_id)
          .with_setter(set_frame_tx_new_connection_id),
        napi::bindgen_prelude::Property::new("frameTxNewToken")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_new_token)
          .with_setter(set_frame_tx_new_token),
        napi::bindgen_prelude::Property::new("frameTxPathChallenge")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_path_challenge)
          .with_setter(set_frame_tx_path_challenge),
        napi::bindgen_prelude::Property::new("frameTxPathResponse")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_path_response)
          .with_setter(set_frame_tx_path_response),
        napi::bindgen_prelude::Property::new("frameTxPing")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_ping)
          .with_setter(set_frame_tx_ping),
        napi::bindgen_prelude::Property::new("frameTxResetStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_reset_stream)
          .with_setter(set_frame_tx_reset_stream),
        napi::bindgen_prelude::Property::new("frameTxRetireConnectionId")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_retire_connection_id)
          .with_setter(set_frame_tx_retire_connection_id),
        napi::bindgen_prelude::Property::new("frameTxStreamDataBlocked")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_stream_data_blocked)
          .with_setter(set_frame_tx_stream_data_blocked),
        napi::bindgen_prelude::Property::new("frameTxStreamsBlockedBidi")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_streams_blocked_bidi)
          .with_setter(set_frame_tx_streams_blocked_bidi),
        napi::bindgen_prelude::Property::new("frameTxStreamsBlockedUni")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_streams_blocked_uni)
          .with_setter(set_frame_tx_streams_blocked_uni),
        napi::bindgen_prelude::Property::new("frameTxStopSending")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_stop_sending)
          .with_setter(set_frame_tx_stop_sending),
        napi::bindgen_prelude::Property::new("frameTxStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_stream)
          .with_setter(set_frame_tx_stream),
        napi::bindgen_prelude::Property::new("frameRxAcks")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_acks)
          .with_setter(set_frame_rx_acks),
        napi::bindgen_prelude::Property::new("frameRxAckFrequency")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_ack_frequency)
          .with_setter(set_frame_rx_ack_frequency),
        napi::bindgen_prelude::Property::new("frameRxCrypto")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_crypto)
          .with_setter(set_frame_rx_crypto),
        napi::bindgen_prelude::Property::new("frameRxConnectionClose")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_connection_close)
          .with_setter(set_frame_rx_connection_close),
        napi::bindgen_prelude::Property::new("frameRxDataBlocked")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_data_blocked)
          .with_setter(set_frame_rx_data_blocked),
        napi::bindgen_prelude::Property::new("frameRxDatagram")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_datagram)
          .with_setter(set_frame_rx_datagram),
        napi::bindgen_prelude::Property::new("frameRxHandshakeDone")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_handshake_done)
          .with_setter(set_frame_rx_handshake_done),
        napi::bindgen_prelude::Property::new("frameRxImmediateAck")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_immediate_ack)
          .with_setter(set_frame_rx_immediate_ack),
        napi::bindgen_prelude::Property::new("frameRxMaxData")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_max_data)
          .with_setter(set_frame_rx_max_data),
        napi::bindgen_prelude::Property::new("frameRxMaxStreamData")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_max_stream_data)
          .with_setter(set_frame_rx_max_stream_data),
        napi::bindgen_prelude::Property::new("frameRxMaxStreamsBidi")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_max_streams_bidi)
          .with_setter(set_frame_rx_max_streams_bidi),
        napi::bindgen_prelude::Property::new("frameRxMaxStreamsUni")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_max_streams_uni)
          .with_setter(set_frame_rx_max_streams_uni),
        napi::bindgen_prelude::Property::new("frameRxNewConnectionId")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_new_connection_id)
          .with_setter(set_frame_rx_new_connection_id),
        napi::bindgen_prelude::Property::new("frameRxNewToken")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_new_token)
          .with_setter(set_frame_rx_new_token),
        napi::bindgen_prelude::Property::new("frameRxPathChallenge")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_path_challenge)
          .with_setter(set_frame_rx_path_challenge),
        napi::bindgen_prelude::Property::new("frameRxPathResponse")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_path_response)
          .with_setter(set_frame_rx_path_response),
        napi::bindgen_prelude::Property::new("frameRxPing")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_ping)
          .with_setter(set_frame_rx_ping),
        napi::bindgen_prelude::Property::new("frameRxResetStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_reset_stream)
          .with_setter(set_frame_rx_reset_stream),
        napi::bindgen_prelude::Property::new("frameRxRetireConnectionId")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_retire_connection_id)
          .with_setter(set_frame_rx_retire_connection_id),
        napi::bindgen_prelude::Property::new("frameRxStreamDataBlocked")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_stream_data_blocked)
          .with_setter(set_frame_rx_stream_data_blocked),
        napi::bindgen_prelude::Property::new("frameRxStreamsBlockedBidi")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_streams_blocked_bidi)
          .with_setter(set_frame_rx_streams_blocked_bidi),
        napi::bindgen_prelude::Property::new("frameRxStreamsBlockedUni")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_streams_blocked_uni)
          .with_setter(set_frame_rx_streams_blocked_uni),
        napi::bindgen_prelude::Property::new("frameRxStopSending")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_stop_sending)
          .with_setter(set_frame_rx_stop_sending),
        napi::bindgen_prelude::Property::new("frameRxStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_stream)
          .with_setter(set_frame_rx_stream),
        napi::bindgen_prelude::Property::new("pathRtt")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_rtt)
          .with_setter(set_path_rtt),
        napi::bindgen_prelude::Property::new("pathCwnd")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_cwnd)
          .with_setter(set_path_cwnd),
        napi::bindgen_prelude::Property::new("pathCongestionEvents")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_congestion_events)
          .with_setter(set_path_congestion_events),
        napi::bindgen_prelude::Property::new("pathLostPackets")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_lost_packets)
          .with_setter(set_path_lost_packets),
        napi::bindgen_prelude::Property::new("pathLostBytes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_lost_bytes)
          .with_setter(set_path_lost_bytes),
        napi::bindgen_prelude::Property::new("pathSentPackets")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_sent_packets)
          .with_setter(set_path_sent_packets),
        napi::bindgen_prelude::Property::new("pathSentPlpmtudProbes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_sent_plpmtud_probes)
          .with_setter(set_path_sent_plpmtud_probes),
        napi::bindgen_prelude::Property::new("pathLostPlpmtudProbes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_lost_plpmtud_probes)
          .with_setter(set_path_lost_plpmtud_probes),
        napi::bindgen_prelude::Property::new("pathBlackHolesDetected")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_black_holes_detected)
          .with_setter(set_path_black_holes_detected),
      ],
    );
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__ConnectionStats_struct_5() {
    napi::__private::register_class(
      "ConnectionStats",
      None,
      "ConnectionStats\0",
      vec![
        napi::bindgen_prelude::Property::new("udpTxDatagrams")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_tx_datagrams)
          .with_setter(set_udp_tx_datagrams),
        napi::bindgen_prelude::Property::new("udpTxBytes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_tx_bytes)
          .with_setter(set_udp_tx_bytes),
        napi::bindgen_prelude::Property::new("udpTxIos")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_tx_ios)
          .with_setter(set_udp_tx_ios),
        napi::bindgen_prelude::Property::new("udpRxDatagrams")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_rx_datagrams)
          .with_setter(set_udp_rx_datagrams),
        napi::bindgen_prelude::Property::new("udpRxBytes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_rx_bytes)
          .with_setter(set_udp_rx_bytes),
        napi::bindgen_prelude::Property::new("udpRxIos")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_udp_rx_ios)
          .with_setter(set_udp_rx_ios),
        napi::bindgen_prelude::Property::new("frameTxAcks")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_acks)
          .with_setter(set_frame_tx_acks),
        napi::bindgen_prelude::Property::new("frameTxAckFrequency")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_ack_frequency)
          .with_setter(set_frame_tx_ack_frequency),
        napi::bindgen_prelude::Property::new("frameTxCrypto")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_crypto)
          .with_setter(set_frame_tx_crypto),
        napi::bindgen_prelude::Property::new("frameTxConnectionClose")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_connection_close)
          .with_setter(set_frame_tx_connection_close),
        napi::bindgen_prelude::Property::new("frameTxDataBlocked")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_data_blocked)
          .with_setter(set_frame_tx_data_blocked),
        napi::bindgen_prelude::Property::new("frameTxDatagram")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_datagram)
          .with_setter(set_frame_tx_datagram),
        napi::bindgen_prelude::Property::new("frameTxHandshakeDone")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_handshake_done)
          .with_setter(set_frame_tx_handshake_done),
        napi::bindgen_prelude::Property::new("frameTxImmediateAck")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_immediate_ack)
          .with_setter(set_frame_tx_immediate_ack),
        napi::bindgen_prelude::Property::new("frameTxMaxData")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_max_data)
          .with_setter(set_frame_tx_max_data),
        napi::bindgen_prelude::Property::new("frameTxMaxStreamData")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_max_stream_data)
          .with_setter(set_frame_tx_max_stream_data),
        napi::bindgen_prelude::Property::new("frameTxMaxStreamsBidi")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_max_streams_bidi)
          .with_setter(set_frame_tx_max_streams_bidi),
        napi::bindgen_prelude::Property::new("frameTxMaxStreamsUni")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_max_streams_uni)
          .with_setter(set_frame_tx_max_streams_uni),
        napi::bindgen_prelude::Property::new("frameTxNewConnectionId")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_new_connection_id)
          .with_setter(set_frame_tx_new_connection_id),
        napi::bindgen_prelude::Property::new("frameTxNewToken")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_new_token)
          .with_setter(set_frame_tx_new_token),
        napi::bindgen_prelude::Property::new("frameTxPathChallenge")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_path_challenge)
          .with_setter(set_frame_tx_path_challenge),
        napi::bindgen_prelude::Property::new("frameTxPathResponse")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_path_response)
          .with_setter(set_frame_tx_path_response),
        napi::bindgen_prelude::Property::new("frameTxPing")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_ping)
          .with_setter(set_frame_tx_ping),
        napi::bindgen_prelude::Property::new("frameTxResetStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_reset_stream)
          .with_setter(set_frame_tx_reset_stream),
        napi::bindgen_prelude::Property::new("frameTxRetireConnectionId")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_retire_connection_id)
          .with_setter(set_frame_tx_retire_connection_id),
        napi::bindgen_prelude::Property::new("frameTxStreamDataBlocked")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_stream_data_blocked)
          .with_setter(set_frame_tx_stream_data_blocked),
        napi::bindgen_prelude::Property::new("frameTxStreamsBlockedBidi")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_streams_blocked_bidi)
          .with_setter(set_frame_tx_streams_blocked_bidi),
        napi::bindgen_prelude::Property::new("frameTxStreamsBlockedUni")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_streams_blocked_uni)
          .with_setter(set_frame_tx_streams_blocked_uni),
        napi::bindgen_prelude::Property::new("frameTxStopSending")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_stop_sending)
          .with_setter(set_frame_tx_stop_sending),
        napi::bindgen_prelude::Property::new("frameTxStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_tx_stream)
          .with_setter(set_frame_tx_stream),
        napi::bindgen_prelude::Property::new("frameRxAcks")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_acks)
          .with_setter(set_frame_rx_acks),
        napi::bindgen_prelude::Property::new("frameRxAckFrequency")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_ack_frequency)
          .with_setter(set_frame_rx_ack_frequency),
        napi::bindgen_prelude::Property::new("frameRxCrypto")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_crypto)
          .with_setter(set_frame_rx_crypto),
        napi::bindgen_prelude::Property::new("frameRxConnectionClose")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_connection_close)
          .with_setter(set_frame_rx_connection_close),
        napi::bindgen_prelude::Property::new("frameRxDataBlocked")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_data_blocked)
          .with_setter(set_frame_rx_data_blocked),
        napi::bindgen_prelude::Property::new("frameRxDatagram")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_datagram)
          .with_setter(set_frame_rx_datagram),
        napi::bindgen_prelude::Property::new("frameRxHandshakeDone")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_handshake_done)
          .with_setter(set_frame_rx_handshake_done),
        napi::bindgen_prelude::Property::new("frameRxImmediateAck")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_immediate_ack)
          .with_setter(set_frame_rx_immediate_ack),
        napi::bindgen_prelude::Property::new("frameRxMaxData")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_max_data)
          .with_setter(set_frame_rx_max_data),
        napi::bindgen_prelude::Property::new("frameRxMaxStreamData")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_max_stream_data)
          .with_setter(set_frame_rx_max_stream_data),
        napi::bindgen_prelude::Property::new("frameRxMaxStreamsBidi")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_max_streams_bidi)
          .with_setter(set_frame_rx_max_streams_bidi),
        napi::bindgen_prelude::Property::new("frameRxMaxStreamsUni")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_max_streams_uni)
          .with_setter(set_frame_rx_max_streams_uni),
        napi::bindgen_prelude::Property::new("frameRxNewConnectionId")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_new_connection_id)
          .with_setter(set_frame_rx_new_connection_id),
        napi::bindgen_prelude::Property::new("frameRxNewToken")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_new_token)
          .with_setter(set_frame_rx_new_token),
        napi::bindgen_prelude::Property::new("frameRxPathChallenge")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_path_challenge)
          .with_setter(set_frame_rx_path_challenge),
        napi::bindgen_prelude::Property::new("frameRxPathResponse")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_path_response)
          .with_setter(set_frame_rx_path_response),
        napi::bindgen_prelude::Property::new("frameRxPing")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_ping)
          .with_setter(set_frame_rx_ping),
        napi::bindgen_prelude::Property::new("frameRxResetStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_reset_stream)
          .with_setter(set_frame_rx_reset_stream),
        napi::bindgen_prelude::Property::new("frameRxRetireConnectionId")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_retire_connection_id)
          .with_setter(set_frame_rx_retire_connection_id),
        napi::bindgen_prelude::Property::new("frameRxStreamDataBlocked")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_stream_data_blocked)
          .with_setter(set_frame_rx_stream_data_blocked),
        napi::bindgen_prelude::Property::new("frameRxStreamsBlockedBidi")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_streams_blocked_bidi)
          .with_setter(set_frame_rx_streams_blocked_bidi),
        napi::bindgen_prelude::Property::new("frameRxStreamsBlockedUni")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_streams_blocked_uni)
          .with_setter(set_frame_rx_streams_blocked_uni),
        napi::bindgen_prelude::Property::new("frameRxStopSending")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_stop_sending)
          .with_setter(set_frame_rx_stop_sending),
        napi::bindgen_prelude::Property::new("frameRxStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_frame_rx_stream)
          .with_setter(set_frame_rx_stream),
        napi::bindgen_prelude::Property::new("pathRtt")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_rtt)
          .with_setter(set_path_rtt),
        napi::bindgen_prelude::Property::new("pathCwnd")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_cwnd)
          .with_setter(set_path_cwnd),
        napi::bindgen_prelude::Property::new("pathCongestionEvents")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_congestion_events)
          .with_setter(set_path_congestion_events),
        napi::bindgen_prelude::Property::new("pathLostPackets")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_lost_packets)
          .with_setter(set_path_lost_packets),
        napi::bindgen_prelude::Property::new("pathLostBytes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_lost_bytes)
          .with_setter(set_path_lost_bytes),
        napi::bindgen_prelude::Property::new("pathSentPackets")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_sent_packets)
          .with_setter(set_path_sent_packets),
        napi::bindgen_prelude::Property::new("pathSentPlpmtudProbes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_sent_plpmtud_probes)
          .with_setter(set_path_sent_plpmtud_probes),
        napi::bindgen_prelude::Property::new("pathLostPlpmtudProbes")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_lost_plpmtud_probes)
          .with_setter(set_path_lost_plpmtud_probes),
        napi::bindgen_prelude::Property::new("pathBlackHolesDetected")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_getter(get_path_black_holes_detected)
          .with_setter(set_path_black_holes_detected),
      ],
    );
  }
}
#[derive(Copy, Clone)]
pub enum SocketFamily {
  Ipv4,
  Ipv6,
}
impl napi::bindgen_prelude::TypeName for SocketFamily {
  fn type_name() -> &'static str {
    "SocketFamily"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for SocketFamily {
  unsafe fn validate(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<napi::sys::napi_value> {
    napi::bindgen_prelude::assert_type_of!(
      env,
      napi_val,
      napi::bindgen_prelude::ValueType::Number
    )?;
    Ok(std::ptr::null_mut())
  }
}
impl napi::bindgen_prelude::FromNapiValue for SocketFamily {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    let val =
      napi::bindgen_prelude::FromNapiValue::from_napi_value(env, napi_val).map_err(|e| {
        napi::bindgen_prelude::error!(
          e.status,
          "Failed to convert napi value into enum `{}`. {}",
          "SocketFamily",
          e,
        )
      })?;
    match val {
      0 => Ok(SocketFamily::Ipv4),
      1 => Ok(SocketFamily::Ipv6),
      _ => Err(napi::bindgen_prelude::error!(
        napi::bindgen_prelude::Status::InvalidArg,
        "value `{:?}` does not match any variant of enum `{}`",
        val,
        "SocketFamily"
      )),
    }
  }
}
impl napi::bindgen_prelude::ToNapiValue for SocketFamily {
  unsafe fn to_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    val: Self,
  ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
    let val = match val {
      SocketFamily::Ipv4 => 0,
      SocketFamily::Ipv6 => 1,
    };
    napi::bindgen_prelude::ToNapiValue::to_napi_value(env, val)
  }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
unsafe fn __register__enum__SocketFamily_callback__(
  env: napi::bindgen_prelude::sys::napi_env,
) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
  use std::ffi::CString;
  use std::ptr;
  let mut obj_ptr = ptr::null_mut();
  napi::bindgen_prelude::check_status!(
    napi::bindgen_prelude::sys::napi_create_object(env, &mut obj_ptr),
    "Failed to create napi object"
  )?;
  {
    let name = std::ffi::CStr::from_bytes_with_nul_unchecked("Ipv4\0".as_bytes());
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_set_named_property(
        env,
        obj_ptr,
        name.as_ptr(),
        napi::bindgen_prelude::ToNapiValue::to_napi_value(env, 0)?
      ),
      "Failed to defined enum `{}`",
      "SocketFamily\0"
    )?;
  };
  {
    let name = std::ffi::CStr::from_bytes_with_nul_unchecked("Ipv6\0".as_bytes());
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_set_named_property(
        env,
        obj_ptr,
        name.as_ptr(),
        napi::bindgen_prelude::ToNapiValue::to_napi_value(env, 1)?
      ),
      "Failed to defined enum `{}`",
      "SocketFamily\0"
    )?;
  };
  Ok(obj_ptr)
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
#[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
#[napi::bindgen_prelude::ctor]
fn __napi_register__SocketFamily_6() {
  napi::bindgen_prelude::register_module_export(
    None,
    "SocketFamily\0",
    __register__enum__SocketFamily_callback__,
  );
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
#[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
#[no_mangle]
extern "C" fn __napi_register__SocketFamily_6() {
  napi::bindgen_prelude::register_module_export(
    None,
    "SocketFamily\0",
    __register__enum__SocketFamily_callback__,
  );
}
pub struct Server {
  socket: Arc<socket::UdpSocket>,
  endpoint: quinn::Endpoint,
}
impl napi::bindgen_prelude::TypeName for Server {
  fn type_name() -> &'static str {
    "Server"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Function
  }
}
impl napi::bindgen_prelude::TypeName for &Server {
  fn type_name() -> &'static str {
    "Server"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::TypeName for &mut Server {
  fn type_name() -> &'static str {
    "Server"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::ToNapiValue for Server {
  unsafe fn to_napi_value(
    env: napi::sys::napi_env,
    val: Server,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    if let Some(ctor_ref) = napi::__private::get_class_constructor("Server\0") {
      let wrapped_value = Box::into_raw(Box::new(val));
      let instance_value = Server::new_instance(env, wrapped_value.cast(), ctor_ref)?;
      Ok(instance_value)
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!(
          "Failed to get constructor of class `{}` in `ToNapiValue`",
          "Server"
        ),
      ))
    }
  }
}
impl napi::bindgen_prelude::ObjectFinalize for Server {}
impl Server {
  pub fn instance_of<V: napi::NapiRaw>(env: napi::Env, value: V) -> napi::Result<bool> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Server\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        unsafe { napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor) },
        "Failed to get constructor reference of class `{}`",
        "Server\0"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        unsafe { napi::sys::napi_instanceof(env.raw(), value.raw(), ctor, &mut is_instance_of) },
        "Failed to run instanceof for class `{}`",
        "Server\0"
      )?;
      Ok(is_instance_of)
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to get constructor of class `{}`", "Server\0"),
      ))
    }
  }
}
impl Server {
  pub fn into_reference(
    val: Server,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::Reference<Server>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Server\0") {
      unsafe {
        let wrapped_value = Box::into_raw(Box::new(val));
        let instance_value = Server::new_instance(env.raw(), wrapped_value.cast(), ctor_ref)?;
        {
          let env = env.raw();
        }
        napi::bindgen_prelude::Reference::<Server>::from_value_ptr(wrapped_value.cast(), env.raw())
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Server"),
      ))
    }
  }
  pub fn into_instance(
    self,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::ClassInstance<Server>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Server\0") {
      unsafe {
        let wrapped_value = Box::leak(Box::new(self));
        let instance_value = Server::new_instance(
          env.raw(),
          wrapped_value as *mut _ as *mut std::ffi::c_void,
          ctor_ref,
        )?;
        Ok(napi::bindgen_prelude::ClassInstance::<Server>::new(
          instance_value,
          wrapped_value,
        ))
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Server"),
      ))
    }
  }
  unsafe fn new_instance(
    env: napi::sys::napi_env,
    wrapped_value: *mut std::ffi::c_void,
    ctor_ref: napi::sys::napi_ref,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut ctor = std::ptr::null_mut();
    napi::check_status!(
      napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
      "Failed to get constructor reference of class `{}`",
      "Server"
    )?;
    let mut result = std::ptr::null_mut();
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
    napi::check_status!(
      napi::sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result),
      "Failed to construct class `{}`",
      "Server"
    )?;
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
    let mut object_ref = std::ptr::null_mut();
    let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
    let finalize_callbacks_ptr = std::rc::Rc::into_raw(std::rc::Rc::new(std::cell::Cell::new(
      Box::into_raw(initial_finalize),
    )));
    napi::check_status!(
      napi::sys::napi_wrap(
        env,
        result,
        wrapped_value,
        Some(napi::bindgen_prelude::raw_finalize_unchecked::<Server>),
        std::ptr::null_mut(),
        &mut object_ref,
      ),
      "Failed to wrap native object of class `{}`",
      "Server"
    )?;
    napi::bindgen_prelude::Reference::<Server>::add_ref(
      env,
      wrapped_value,
      (wrapped_value, object_ref, finalize_callbacks_ptr),
    );
    Ok(result)
  }
}
impl napi::bindgen_prelude::FromNapiRef for Server {
  unsafe fn from_napi_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "Server",
    )?;
    Ok(&*(wrapped_val as *const Server))
  }
}
impl napi::bindgen_prelude::FromNapiMutRef for Server {
  unsafe fn from_napi_mut_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static mut Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "Server",
    )?;
    Ok(&mut *(wrapped_val as *mut Server))
  }
}
impl napi::bindgen_prelude::FromNapiValue for &Server {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::FromNapiValue for &mut Server {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &Server {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Server\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "Server"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "Server"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "Server"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Server"),
      ))
    }
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &mut Server {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Server\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "Server"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "Server"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "Server"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Server"),
      ))
    }
  }
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
mod __napi_helper__Server {
  use super::*;
  use std::ptr;
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__Server_struct_7() {
    napi::__private::register_class("Server", None, "Server\0", vec![]);
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__Server_struct_7() {
    napi::__private::register_class("Server", None, "Server\0", vec![]);
  }
}
impl Server {
  pub fn new(config: &config::QuinnConfig, ip: String, port: u16) -> Result<Self> {
    let ip_addr = ip.parse::<IpAddr>().map_err(to_err)?;
    let socket_addr = SocketAddr::new(ip_addr, port);
    let socket = std::net::UdpSocket::bind(socket_addr)?;
    let socket = socket::UdpSocket::wrap_udp_socket(socket)?;
    let endpoint = quinn::Endpoint::new_with_abstract_socket(
      config.endpoint_config.clone(),
      Some(config.server_config.clone()),
      socket.clone(),
      Arc::new(quinn::TokioRuntime),
    )?;
    Ok(Self { socket, endpoint })
  }
  pub async fn inbound_connection(&self) -> Result<Connection> {
    let incoming = match self.endpoint.accept().await {
      Some(incoming) => Ok(incoming),
      None => Err(to_err("server closed")),
    }?;
    let connection = incoming.await.map_err(to_err)?;
    Ok(Connection::new(connection))
  }
  pub async unsafe fn abort(&mut self) {
    self.endpoint.close(0u8.into(), b"");
    self.endpoint.wait_idle().await;
    self.socket.unbind().await;
  }
  pub fn stats(&self) -> stats::EndpointStats {
    let stats = self.endpoint.stats();
    stats::EndpointStats::new(
      stats.accepted_handshakes,
      stats.outgoing_handshakes,
      stats.refused_handshakes,
      stats.ignored_handshakes,
    )
  }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod __napi_impl_helper__Server__1 {
  use super::*;
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__new(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      if napi::__private::___CALL_FROM_FACTORY
        .with(|inner| inner.load(std::sync::atomic::Ordering::Relaxed))
      {
        return std::ptr::null_mut();
      }
      napi::bindgen_prelude::CallbackInfo::<3usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let arg0 = {
            <config::QuinnConfig as napi::bindgen_prelude::FromNapiRef>::from_napi_ref(
              env,
              cb.get_arg(0usize),
            )?
          };
          let arg1 = {
            <String as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
              env,
              cb.get_arg(1usize),
            )?
          };
          let arg2 = {
            <u16 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb.get_arg(2usize))?
          };
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { Server::new(arg0, arg1, arg2) };
            match _ret {
              Ok(value) => cb.construct::<false, Server>("constructor", value),
              Err(err) => {
                napi::bindgen_prelude::JsError::from(err).throw_into(env);
                Ok(std::ptr::null_mut())
              }
            }
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__inbound_connection(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
          impl NapiRefContainer {
            fn drop(self, env: napi::sys::napi_env) {
              for r in self.0.into_iter() {
                assert_eq!(
                  unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
                assert_eq!(
                  unsafe { napi::sys::napi_delete_reference(env, r) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
              }
            }
          }
          unsafe impl Send for NapiRefContainer {}
          unsafe impl Sync for NapiRefContainer {}
          let _make_ref = |a: ::std::ptr::NonNull<napi::bindgen_prelude::sys::napi_value__>| {
            let mut node_ref = ::std::mem::MaybeUninit::uninit();
            napi::bindgen_prelude::check_status!(
              unsafe {
                napi::bindgen_prelude::sys::napi_create_reference(
                  env,
                  a.as_ptr(),
                  1,
                  node_ref.as_mut_ptr(),
                )
              },
              "failed to create napi ref"
            )?;
            Ok::<napi::sys::napi_ref, napi::Error>(unsafe { node_ref.assume_init() })
          };
          let mut _args_array =
            [::std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_ref__>(); 1usize];
          let mut _arg_write_index = 0;
          _args_array[_arg_write_index] =
            _make_ref(::std::ptr::NonNull::new(cb.this).ok_or_else(|| {
              napi::Error::new(
                napi::Status::InvalidArg,
                "referenced ptr is null".to_owned(),
              )
            })?)?;
          _arg_write_index += 1;
          #[cfg(debug_assert)]
          {
            for a in &_args_array {
              assert!(!a.is_null(), "failed to initialize napi ref");
            }
          }
          let _args_ref = NapiRefContainer(_args_array);
          let this_ptr = unsafe { cb.unwrap_raw::<Server>()? };
          let this: &Server = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::execute_tokio_future(
            env,
            async move { this.inbound_connection().await },
            move |env, _ret| {
              _args_ref.drop(env);
              <Connection as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
            },
          )
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__abort(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
          impl NapiRefContainer {
            fn drop(self, env: napi::sys::napi_env) {
              for r in self.0.into_iter() {
                assert_eq!(
                  unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
                assert_eq!(
                  unsafe { napi::sys::napi_delete_reference(env, r) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
              }
            }
          }
          unsafe impl Send for NapiRefContainer {}
          unsafe impl Sync for NapiRefContainer {}
          let _make_ref = |a: ::std::ptr::NonNull<napi::bindgen_prelude::sys::napi_value__>| {
            let mut node_ref = ::std::mem::MaybeUninit::uninit();
            napi::bindgen_prelude::check_status!(
              unsafe {
                napi::bindgen_prelude::sys::napi_create_reference(
                  env,
                  a.as_ptr(),
                  1,
                  node_ref.as_mut_ptr(),
                )
              },
              "failed to create napi ref"
            )?;
            Ok::<napi::sys::napi_ref, napi::Error>(unsafe { node_ref.assume_init() })
          };
          let mut _args_array =
            [::std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_ref__>(); 1usize];
          let mut _arg_write_index = 0;
          _args_array[_arg_write_index] =
            _make_ref(::std::ptr::NonNull::new(cb.this).ok_or_else(|| {
              napi::Error::new(
                napi::Status::InvalidArg,
                "referenced ptr is null".to_owned(),
              )
            })?)?;
          _arg_write_index += 1;
          #[cfg(debug_assert)]
          {
            for a in &_args_array {
              assert!(!a.is_null(), "failed to initialize napi ref");
            }
          }
          let _args_ref = NapiRefContainer(_args_array);
          let this_ptr = unsafe { cb.unwrap_raw::<Server>()? };
          let this: &mut Server = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::execute_tokio_future(
            env,
            async move { Ok(this.abort().await) },
            move |env, _ret| {
              _args_ref.drop(env);
              <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
            },
          )
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__stats(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Server>()? };
          let this: &Server = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.stats() };
            <stats::EndpointStats as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__Server_impl_12() {
    napi::__private::register_class(
      "Server",
      None,
      "Server\0",
      vec![
        napi::bindgen_prelude::Property::new("abort")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__abort),
        napi::bindgen_prelude::Property::new("constructor")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_ctor(__napi__new),
        napi::bindgen_prelude::Property::new("inboundConnection")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__inbound_connection),
        napi::bindgen_prelude::Property::new("stats")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__stats),
      ],
    );
  }
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__Server_impl_12() {
    napi::__private::register_class(
      "Server",
      None,
      "Server\0",
      vec![
        napi::bindgen_prelude::Property::new("abort")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__abort),
        napi::bindgen_prelude::Property::new("constructor")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_ctor(__napi__new),
        napi::bindgen_prelude::Property::new("inboundConnection")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__inbound_connection),
        napi::bindgen_prelude::Property::new("stats")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__stats),
      ],
    );
  }
}
pub struct Client {
  endpoint: quinn::Endpoint,
}
impl napi::bindgen_prelude::TypeName for Client {
  fn type_name() -> &'static str {
    "Client"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Function
  }
}
impl napi::bindgen_prelude::TypeName for &Client {
  fn type_name() -> &'static str {
    "Client"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::TypeName for &mut Client {
  fn type_name() -> &'static str {
    "Client"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::ToNapiValue for Client {
  unsafe fn to_napi_value(
    env: napi::sys::napi_env,
    val: Client,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    if let Some(ctor_ref) = napi::__private::get_class_constructor("Client\0") {
      let wrapped_value = Box::into_raw(Box::new(val));
      let instance_value = Client::new_instance(env, wrapped_value.cast(), ctor_ref)?;
      Ok(instance_value)
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!(
          "Failed to get constructor of class `{}` in `ToNapiValue`",
          "Client"
        ),
      ))
    }
  }
}
impl napi::bindgen_prelude::ObjectFinalize for Client {}
impl Client {
  pub fn instance_of<V: napi::NapiRaw>(env: napi::Env, value: V) -> napi::Result<bool> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Client\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        unsafe { napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor) },
        "Failed to get constructor reference of class `{}`",
        "Client\0"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        unsafe { napi::sys::napi_instanceof(env.raw(), value.raw(), ctor, &mut is_instance_of) },
        "Failed to run instanceof for class `{}`",
        "Client\0"
      )?;
      Ok(is_instance_of)
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to get constructor of class `{}`", "Client\0"),
      ))
    }
  }
}
impl Client {
  pub fn into_reference(
    val: Client,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::Reference<Client>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Client\0") {
      unsafe {
        let wrapped_value = Box::into_raw(Box::new(val));
        let instance_value = Client::new_instance(env.raw(), wrapped_value.cast(), ctor_ref)?;
        {
          let env = env.raw();
        }
        napi::bindgen_prelude::Reference::<Client>::from_value_ptr(wrapped_value.cast(), env.raw())
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Client"),
      ))
    }
  }
  pub fn into_instance(
    self,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::ClassInstance<Client>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Client\0") {
      unsafe {
        let wrapped_value = Box::leak(Box::new(self));
        let instance_value = Client::new_instance(
          env.raw(),
          wrapped_value as *mut _ as *mut std::ffi::c_void,
          ctor_ref,
        )?;
        Ok(napi::bindgen_prelude::ClassInstance::<Client>::new(
          instance_value,
          wrapped_value,
        ))
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Client"),
      ))
    }
  }
  unsafe fn new_instance(
    env: napi::sys::napi_env,
    wrapped_value: *mut std::ffi::c_void,
    ctor_ref: napi::sys::napi_ref,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut ctor = std::ptr::null_mut();
    napi::check_status!(
      napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
      "Failed to get constructor reference of class `{}`",
      "Client"
    )?;
    let mut result = std::ptr::null_mut();
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
    napi::check_status!(
      napi::sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result),
      "Failed to construct class `{}`",
      "Client"
    )?;
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
    let mut object_ref = std::ptr::null_mut();
    let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
    let finalize_callbacks_ptr = std::rc::Rc::into_raw(std::rc::Rc::new(std::cell::Cell::new(
      Box::into_raw(initial_finalize),
    )));
    napi::check_status!(
      napi::sys::napi_wrap(
        env,
        result,
        wrapped_value,
        Some(napi::bindgen_prelude::raw_finalize_unchecked::<Client>),
        std::ptr::null_mut(),
        &mut object_ref,
      ),
      "Failed to wrap native object of class `{}`",
      "Client"
    )?;
    napi::bindgen_prelude::Reference::<Client>::add_ref(
      env,
      wrapped_value,
      (wrapped_value, object_ref, finalize_callbacks_ptr),
    );
    Ok(result)
  }
}
impl napi::bindgen_prelude::FromNapiRef for Client {
  unsafe fn from_napi_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "Client",
    )?;
    Ok(&*(wrapped_val as *const Client))
  }
}
impl napi::bindgen_prelude::FromNapiMutRef for Client {
  unsafe fn from_napi_mut_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static mut Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "Client",
    )?;
    Ok(&mut *(wrapped_val as *mut Client))
  }
}
impl napi::bindgen_prelude::FromNapiValue for &Client {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::FromNapiValue for &mut Client {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &Client {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Client\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "Client"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "Client"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "Client"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Client"),
      ))
    }
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &mut Client {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Client\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "Client"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "Client"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "Client"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Client"),
      ))
    }
  }
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
mod __napi_helper__Client {
  use super::*;
  use std::ptr;
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__Client_struct_13() {
    napi::__private::register_class("Client", None, "Client\0", vec![]);
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__Client_struct_13() {
    napi::__private::register_class("Client", None, "Client\0", vec![]);
  }
}
impl Client {
  pub fn new(config: &config::QuinnConfig, family: SocketFamily) -> Result<Self> {
    let bind_addr = match family {
      SocketFamily::Ipv4 => SocketAddr::new(std::net::Ipv4Addr::UNSPECIFIED.into(), 0),
      SocketFamily::Ipv6 => SocketAddr::new(std::net::Ipv6Addr::UNSPECIFIED.into(), 0),
    };
    let socket = std::net::UdpSocket::bind(bind_addr)?;
    let mut endpoint = quinn::Endpoint::new(
      config.endpoint_config.clone(),
      None,
      socket,
      Arc::new(quinn::TokioRuntime),
    )?;
    endpoint.set_default_client_config(config.client_config.clone());
    Ok(Client { endpoint })
  }
  pub async fn outbound_connection(&self, ip: String, port: u16) -> Result<Connection> {
    let ip_addr = ip.parse::<IpAddr>().map_err(to_err)?;
    let socket_addr = SocketAddr::new(ip_addr, port);
    let connecting = self.endpoint.connect(socket_addr, "l").map_err(to_err)?;
    let connection = connecting.await.map_err(to_err)?;
    Ok(Connection::new(connection))
  }
  pub fn abort(&self) {
    self.endpoint.close(0u8.into(), b"");
  }
  pub fn stats(&self) -> stats::EndpointStats {
    let stats = self.endpoint.stats();
    stats::EndpointStats::new(
      stats.accepted_handshakes,
      stats.outgoing_handshakes,
      stats.refused_handshakes,
      stats.ignored_handshakes,
    )
  }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod __napi_impl_helper__Client__2 {
  use super::*;
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__new(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      if napi::__private::___CALL_FROM_FACTORY
        .with(|inner| inner.load(std::sync::atomic::Ordering::Relaxed))
      {
        return std::ptr::null_mut();
      }
      napi::bindgen_prelude::CallbackInfo::<2usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let arg0 = {
            <config::QuinnConfig as napi::bindgen_prelude::FromNapiRef>::from_napi_ref(
              env,
              cb.get_arg(0usize),
            )?
          };
          let arg1 = {
            <SocketFamily as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
              env,
              cb.get_arg(1usize),
            )?
          };
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { Client::new(arg0, arg1) };
            match _ret {
              Ok(value) => cb.construct::<false, Client>("constructor", value),
              Err(err) => {
                napi::bindgen_prelude::JsError::from(err).throw_into(env);
                Ok(std::ptr::null_mut())
              }
            }
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__outbound_connection(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<2usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
          impl NapiRefContainer {
            fn drop(self, env: napi::sys::napi_env) {
              for r in self.0.into_iter() {
                assert_eq!(
                  unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
                assert_eq!(
                  unsafe { napi::sys::napi_delete_reference(env, r) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
              }
            }
          }
          unsafe impl Send for NapiRefContainer {}
          unsafe impl Sync for NapiRefContainer {}
          let _make_ref = |a: ::std::ptr::NonNull<napi::bindgen_prelude::sys::napi_value__>| {
            let mut node_ref = ::std::mem::MaybeUninit::uninit();
            napi::bindgen_prelude::check_status!(
              unsafe {
                napi::bindgen_prelude::sys::napi_create_reference(
                  env,
                  a.as_ptr(),
                  1,
                  node_ref.as_mut_ptr(),
                )
              },
              "failed to create napi ref"
            )?;
            Ok::<napi::sys::napi_ref, napi::Error>(unsafe { node_ref.assume_init() })
          };
          let mut _args_array =
            [::std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_ref__>(); 1usize];
          let mut _arg_write_index = 0;
          _args_array[_arg_write_index] =
            _make_ref(::std::ptr::NonNull::new(cb.this).ok_or_else(|| {
              napi::Error::new(
                napi::Status::InvalidArg,
                "referenced ptr is null".to_owned(),
              )
            })?)?;
          _arg_write_index += 1;
          #[cfg(debug_assert)]
          {
            for a in &_args_array {
              assert!(!a.is_null(), "failed to initialize napi ref");
            }
          }
          let _args_ref = NapiRefContainer(_args_array);
          let this_ptr = unsafe { cb.unwrap_raw::<Client>()? };
          let this: &Client = Box::leak(Box::from_raw(this_ptr));
          let arg0 = {
            <String as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
              env,
              cb.get_arg(0usize),
            )?
          };
          let arg1 = {
            <u16 as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb.get_arg(1usize))?
          };
          napi::bindgen_prelude::execute_tokio_future(
            env,
            async move { this.outbound_connection(arg0, arg1).await },
            move |env, _ret| {
              _args_ref.drop(env);
              <Connection as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
            },
          )
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__abort(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Client>()? };
          let this: &Client = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.abort() };
            <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__stats(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Client>()? };
          let this: &Client = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.stats() };
            <stats::EndpointStats as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__Client_impl_18() {
    napi::__private::register_class(
      "Client",
      None,
      "Client\0",
      vec![
        napi::bindgen_prelude::Property::new("abort")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__abort),
        napi::bindgen_prelude::Property::new("constructor")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_ctor(__napi__new),
        napi::bindgen_prelude::Property::new("outboundConnection")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__outbound_connection),
        napi::bindgen_prelude::Property::new("stats")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__stats),
      ],
    );
  }
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__Client_impl_18() {
    napi::__private::register_class(
      "Client",
      None,
      "Client\0",
      vec![
        napi::bindgen_prelude::Property::new("abort")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__abort),
        napi::bindgen_prelude::Property::new("constructor")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_ctor(__napi__new),
        napi::bindgen_prelude::Property::new("outboundConnection")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__outbound_connection),
        napi::bindgen_prelude::Property::new("stats")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__stats),
      ],
    );
  }
}
pub struct Connection {
  connection: quinn::Connection,
}
impl napi::bindgen_prelude::TypeName for Connection {
  fn type_name() -> &'static str {
    "Connection"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Function
  }
}
impl napi::bindgen_prelude::TypeName for &Connection {
  fn type_name() -> &'static str {
    "Connection"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::TypeName for &mut Connection {
  fn type_name() -> &'static str {
    "Connection"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::ToNapiValue for Connection {
  unsafe fn to_napi_value(
    env: napi::sys::napi_env,
    val: Connection,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    if let Some(ctor_ref) = napi::__private::get_class_constructor("Connection\0") {
      let wrapped_value = Box::into_raw(Box::new(val));
      let instance_value = Connection::new_instance(env, wrapped_value.cast(), ctor_ref)?;
      Ok(instance_value)
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!(
          "Failed to get constructor of class `{}` in `ToNapiValue`",
          "Connection"
        ),
      ))
    }
  }
}
impl napi::bindgen_prelude::ObjectFinalize for Connection {}
impl Connection {
  pub fn instance_of<V: napi::NapiRaw>(env: napi::Env, value: V) -> napi::Result<bool> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Connection\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        unsafe { napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor) },
        "Failed to get constructor reference of class `{}`",
        "Connection\0"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        unsafe { napi::sys::napi_instanceof(env.raw(), value.raw(), ctor, &mut is_instance_of) },
        "Failed to run instanceof for class `{}`",
        "Connection\0"
      )?;
      Ok(is_instance_of)
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to get constructor of class `{}`", "Connection\0"),
      ))
    }
  }
}
impl Connection {
  pub fn into_reference(
    val: Connection,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::Reference<Connection>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Connection\0") {
      unsafe {
        let wrapped_value = Box::into_raw(Box::new(val));
        let instance_value = Connection::new_instance(env.raw(), wrapped_value.cast(), ctor_ref)?;
        {
          let env = env.raw();
        }
        napi::bindgen_prelude::Reference::<Connection>::from_value_ptr(
          wrapped_value.cast(),
          env.raw(),
        )
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Connection"),
      ))
    }
  }
  pub fn into_instance(
    self,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::ClassInstance<Connection>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Connection\0") {
      unsafe {
        let wrapped_value = Box::leak(Box::new(self));
        let instance_value = Connection::new_instance(
          env.raw(),
          wrapped_value as *mut _ as *mut std::ffi::c_void,
          ctor_ref,
        )?;
        Ok(napi::bindgen_prelude::ClassInstance::<Connection>::new(
          instance_value,
          wrapped_value,
        ))
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Connection"),
      ))
    }
  }
  unsafe fn new_instance(
    env: napi::sys::napi_env,
    wrapped_value: *mut std::ffi::c_void,
    ctor_ref: napi::sys::napi_ref,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut ctor = std::ptr::null_mut();
    napi::check_status!(
      napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
      "Failed to get constructor reference of class `{}`",
      "Connection"
    )?;
    let mut result = std::ptr::null_mut();
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
    napi::check_status!(
      napi::sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result),
      "Failed to construct class `{}`",
      "Connection"
    )?;
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
    let mut object_ref = std::ptr::null_mut();
    let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
    let finalize_callbacks_ptr = std::rc::Rc::into_raw(std::rc::Rc::new(std::cell::Cell::new(
      Box::into_raw(initial_finalize),
    )));
    napi::check_status!(
      napi::sys::napi_wrap(
        env,
        result,
        wrapped_value,
        Some(napi::bindgen_prelude::raw_finalize_unchecked::<Connection>),
        std::ptr::null_mut(),
        &mut object_ref,
      ),
      "Failed to wrap native object of class `{}`",
      "Connection"
    )?;
    napi::bindgen_prelude::Reference::<Connection>::add_ref(
      env,
      wrapped_value,
      (wrapped_value, object_ref, finalize_callbacks_ptr),
    );
    Ok(result)
  }
}
impl napi::bindgen_prelude::FromNapiRef for Connection {
  unsafe fn from_napi_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "Connection",
    )?;
    Ok(&*(wrapped_val as *const Connection))
  }
}
impl napi::bindgen_prelude::FromNapiMutRef for Connection {
  unsafe fn from_napi_mut_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static mut Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "Connection",
    )?;
    Ok(&mut *(wrapped_val as *mut Connection))
  }
}
impl napi::bindgen_prelude::FromNapiValue for &Connection {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::FromNapiValue for &mut Connection {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &Connection {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Connection\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "Connection"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "Connection"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "Connection"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Connection"),
      ))
    }
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &mut Connection {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Connection\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "Connection"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "Connection"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "Connection"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Connection"),
      ))
    }
  }
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
mod __napi_helper__Connection {
  use super::*;
  use std::ptr;
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__Connection_struct_19() {
    napi::__private::register_class("Connection", None, "Connection\0", vec![]);
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__Connection_struct_19() {
    napi::__private::register_class("Connection", None, "Connection\0", vec![]);
  }
}
impl Connection {
  pub fn new(connection: quinn::Connection) -> Self {
    Self { connection }
  }
  pub async fn inbound_stream(&self) -> Result<Stream> {
    let (send, recv) = self.connection.accept_bi().await.map_err(to_err)?;
    Ok(Stream::new(send, recv))
  }
  pub async fn outbound_stream(&self) -> Result<Stream> {
    let (send, recv) = self.connection.open_bi().await.map_err(to_err)?;
    Ok(Stream::new(send, recv))
  }
  #[doc = " close the connection immediately"]
  pub fn abort(&self) {
    self.connection.close(0u8.into(), b"");
  }
  pub fn rtt(&self) -> u32 {
    self.connection.rtt().as_millis() as u32
  }
  pub fn id(&self) -> String {
    self.connection.stable_id().to_string()
  }
  pub fn remote_multiaddr(&self) -> String {
    let remote_addr = self.connection.remote_address();
    format!(
      "/{}/{}/udp/{}/quic-v1/p2p/{}",
      match remote_addr {
        SocketAddr::V4(_) => "ip4",
        SocketAddr::V6(_) => "ip6",
      },
      remote_addr.ip(),
      remote_addr.port(),
      self.peer_id().to_base58()
    )
  }
  pub fn peer_id(&self) -> libp2p_identity::PeerId {
    let identity = self
      .connection
      .peer_identity()
      .expect("connection got identity because it passed TLS handshake; qed");
    let certificates: Box<Vec<rustls::pki_types::CertificateDer>> =
      identity.downcast().expect("we rely on rustls feature; qed");
    let end_entity = certificates
      .first()
      .expect("there should be exactly one certificate; qed");
    let p2p_cert = libp2p_tls::certificate::parse(end_entity)
      .expect("the certificate was validated during TLS handshake; qed");
    p2p_cert.peer_id()
  }
  pub async fn closed(&self) -> () {
    self.connection.closed().await;
  }
  pub fn stats(&self) -> stats::ConnectionStats {
    self.connection.stats().into()
  }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod __napi_impl_helper__Connection__3 {
  use super::*;
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__inbound_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
          impl NapiRefContainer {
            fn drop(self, env: napi::sys::napi_env) {
              for r in self.0.into_iter() {
                assert_eq!(
                  unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
                assert_eq!(
                  unsafe { napi::sys::napi_delete_reference(env, r) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
              }
            }
          }
          unsafe impl Send for NapiRefContainer {}
          unsafe impl Sync for NapiRefContainer {}
          let _make_ref = |a: ::std::ptr::NonNull<napi::bindgen_prelude::sys::napi_value__>| {
            let mut node_ref = ::std::mem::MaybeUninit::uninit();
            napi::bindgen_prelude::check_status!(
              unsafe {
                napi::bindgen_prelude::sys::napi_create_reference(
                  env,
                  a.as_ptr(),
                  1,
                  node_ref.as_mut_ptr(),
                )
              },
              "failed to create napi ref"
            )?;
            Ok::<napi::sys::napi_ref, napi::Error>(unsafe { node_ref.assume_init() })
          };
          let mut _args_array =
            [::std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_ref__>(); 1usize];
          let mut _arg_write_index = 0;
          _args_array[_arg_write_index] =
            _make_ref(::std::ptr::NonNull::new(cb.this).ok_or_else(|| {
              napi::Error::new(
                napi::Status::InvalidArg,
                "referenced ptr is null".to_owned(),
              )
            })?)?;
          _arg_write_index += 1;
          #[cfg(debug_assert)]
          {
            for a in &_args_array {
              assert!(!a.is_null(), "failed to initialize napi ref");
            }
          }
          let _args_ref = NapiRefContainer(_args_array);
          let this_ptr = unsafe { cb.unwrap_raw::<Connection>()? };
          let this: &Connection = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::execute_tokio_future(
            env,
            async move { this.inbound_stream().await },
            move |env, _ret| {
              _args_ref.drop(env);
              <Stream as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
            },
          )
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__outbound_stream(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
          impl NapiRefContainer {
            fn drop(self, env: napi::sys::napi_env) {
              for r in self.0.into_iter() {
                assert_eq!(
                  unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
                assert_eq!(
                  unsafe { napi::sys::napi_delete_reference(env, r) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
              }
            }
          }
          unsafe impl Send for NapiRefContainer {}
          unsafe impl Sync for NapiRefContainer {}
          let _make_ref = |a: ::std::ptr::NonNull<napi::bindgen_prelude::sys::napi_value__>| {
            let mut node_ref = ::std::mem::MaybeUninit::uninit();
            napi::bindgen_prelude::check_status!(
              unsafe {
                napi::bindgen_prelude::sys::napi_create_reference(
                  env,
                  a.as_ptr(),
                  1,
                  node_ref.as_mut_ptr(),
                )
              },
              "failed to create napi ref"
            )?;
            Ok::<napi::sys::napi_ref, napi::Error>(unsafe { node_ref.assume_init() })
          };
          let mut _args_array =
            [::std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_ref__>(); 1usize];
          let mut _arg_write_index = 0;
          _args_array[_arg_write_index] =
            _make_ref(::std::ptr::NonNull::new(cb.this).ok_or_else(|| {
              napi::Error::new(
                napi::Status::InvalidArg,
                "referenced ptr is null".to_owned(),
              )
            })?)?;
          _arg_write_index += 1;
          #[cfg(debug_assert)]
          {
            for a in &_args_array {
              assert!(!a.is_null(), "failed to initialize napi ref");
            }
          }
          let _args_ref = NapiRefContainer(_args_array);
          let this_ptr = unsafe { cb.unwrap_raw::<Connection>()? };
          let this: &Connection = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::execute_tokio_future(
            env,
            async move { this.outbound_stream().await },
            move |env, _ret| {
              _args_ref.drop(env);
              <Stream as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
            },
          )
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc = " close the connection immediately"]
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__abort(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Connection>()? };
          let this: &Connection = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.abort() };
            <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__rtt(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Connection>()? };
          let this: &Connection = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.rtt() };
            <u32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Connection>()? };
          let this: &Connection = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.id() };
            <String as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__remote_multiaddr(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Connection>()? };
          let this: &Connection = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.remote_multiaddr() };
            <String as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__closed(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
          impl NapiRefContainer {
            fn drop(self, env: napi::sys::napi_env) {
              for r in self.0.into_iter() {
                assert_eq!(
                  unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
                assert_eq!(
                  unsafe { napi::sys::napi_delete_reference(env, r) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
              }
            }
          }
          unsafe impl Send for NapiRefContainer {}
          unsafe impl Sync for NapiRefContainer {}
          let _make_ref = |a: ::std::ptr::NonNull<napi::bindgen_prelude::sys::napi_value__>| {
            let mut node_ref = ::std::mem::MaybeUninit::uninit();
            napi::bindgen_prelude::check_status!(
              unsafe {
                napi::bindgen_prelude::sys::napi_create_reference(
                  env,
                  a.as_ptr(),
                  1,
                  node_ref.as_mut_ptr(),
                )
              },
              "failed to create napi ref"
            )?;
            Ok::<napi::sys::napi_ref, napi::Error>(unsafe { node_ref.assume_init() })
          };
          let mut _args_array =
            [::std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_ref__>(); 1usize];
          let mut _arg_write_index = 0;
          _args_array[_arg_write_index] =
            _make_ref(::std::ptr::NonNull::new(cb.this).ok_or_else(|| {
              napi::Error::new(
                napi::Status::InvalidArg,
                "referenced ptr is null".to_owned(),
              )
            })?)?;
          _arg_write_index += 1;
          #[cfg(debug_assert)]
          {
            for a in &_args_array {
              assert!(!a.is_null(), "failed to initialize napi ref");
            }
          }
          let _args_ref = NapiRefContainer(_args_array);
          let this_ptr = unsafe { cb.unwrap_raw::<Connection>()? };
          let this: &Connection = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::execute_tokio_future(
            env,
            async move { Ok(this.closed().await) },
            move |env, _ret| {
              _args_ref.drop(env);
              <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
            },
          )
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__stats(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Connection>()? };
          let this: &Connection = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.stats() };
            <stats::ConnectionStats as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__Connection_impl_28() {
    napi::__private::register_class(
      "Connection",
      None,
      "Connection\0",
      vec![
        napi::bindgen_prelude::Property::new("abort")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__abort),
        napi::bindgen_prelude::Property::new("closed")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__closed),
        napi::bindgen_prelude::Property::new("id")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__id),
        napi::bindgen_prelude::Property::new("inboundStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__inbound_stream),
        napi::bindgen_prelude::Property::new("outboundStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__outbound_stream),
        napi::bindgen_prelude::Property::new("remoteMultiaddr")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__remote_multiaddr),
        napi::bindgen_prelude::Property::new("rtt")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__rtt),
        napi::bindgen_prelude::Property::new("stats")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__stats),
      ],
    );
  }
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__Connection_impl_28() {
    napi::__private::register_class(
      "Connection",
      None,
      "Connection\0",
      vec![
        napi::bindgen_prelude::Property::new("abort")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__abort),
        napi::bindgen_prelude::Property::new("closed")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__closed),
        napi::bindgen_prelude::Property::new("id")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__id),
        napi::bindgen_prelude::Property::new("inboundStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__inbound_stream),
        napi::bindgen_prelude::Property::new("outboundStream")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__outbound_stream),
        napi::bindgen_prelude::Property::new("remoteMultiaddr")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__remote_multiaddr),
        napi::bindgen_prelude::Property::new("rtt")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__rtt),
        napi::bindgen_prelude::Property::new("stats")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__stats),
      ],
    );
  }
}
pub struct Stream {
  send: quinn::SendStream,
  recv: quinn::RecvStream,
}
impl napi::bindgen_prelude::TypeName for Stream {
  fn type_name() -> &'static str {
    "Stream"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Function
  }
}
impl napi::bindgen_prelude::TypeName for &Stream {
  fn type_name() -> &'static str {
    "Stream"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::TypeName for &mut Stream {
  fn type_name() -> &'static str {
    "Stream"
  }
  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}
impl napi::bindgen_prelude::ToNapiValue for Stream {
  unsafe fn to_napi_value(
    env: napi::sys::napi_env,
    val: Stream,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    if let Some(ctor_ref) = napi::__private::get_class_constructor("Stream\0") {
      let wrapped_value = Box::into_raw(Box::new(val));
      let instance_value = Stream::new_instance(env, wrapped_value.cast(), ctor_ref)?;
      Ok(instance_value)
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!(
          "Failed to get constructor of class `{}` in `ToNapiValue`",
          "Stream"
        ),
      ))
    }
  }
}
impl napi::bindgen_prelude::ObjectFinalize for Stream {}
impl Stream {
  pub fn instance_of<V: napi::NapiRaw>(env: napi::Env, value: V) -> napi::Result<bool> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Stream\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        unsafe { napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor) },
        "Failed to get constructor reference of class `{}`",
        "Stream\0"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        unsafe { napi::sys::napi_instanceof(env.raw(), value.raw(), ctor, &mut is_instance_of) },
        "Failed to run instanceof for class `{}`",
        "Stream\0"
      )?;
      Ok(is_instance_of)
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to get constructor of class `{}`", "Stream\0"),
      ))
    }
  }
}
impl Stream {
  pub fn into_reference(
    val: Stream,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::Reference<Stream>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Stream\0") {
      unsafe {
        let wrapped_value = Box::into_raw(Box::new(val));
        let instance_value = Stream::new_instance(env.raw(), wrapped_value.cast(), ctor_ref)?;
        {
          let env = env.raw();
        }
        napi::bindgen_prelude::Reference::<Stream>::from_value_ptr(wrapped_value.cast(), env.raw())
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Stream"),
      ))
    }
  }
  pub fn into_instance(
    self,
    env: napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::ClassInstance<Stream>> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Stream\0") {
      unsafe {
        let wrapped_value = Box::leak(Box::new(self));
        let instance_value = Stream::new_instance(
          env.raw(),
          wrapped_value as *mut _ as *mut std::ffi::c_void,
          ctor_ref,
        )?;
        Ok(napi::bindgen_prelude::ClassInstance::<Stream>::new(
          instance_value,
          wrapped_value,
        ))
      }
    } else {
      Err(napi::bindgen_prelude::Error::new(
        napi::bindgen_prelude::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Stream"),
      ))
    }
  }
  unsafe fn new_instance(
    env: napi::sys::napi_env,
    wrapped_value: *mut std::ffi::c_void,
    ctor_ref: napi::sys::napi_ref,
  ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut ctor = std::ptr::null_mut();
    napi::check_status!(
      napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
      "Failed to get constructor reference of class `{}`",
      "Stream"
    )?;
    let mut result = std::ptr::null_mut();
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
    napi::check_status!(
      napi::sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result),
      "Failed to construct class `{}`",
      "Stream"
    )?;
    napi::__private::___CALL_FROM_FACTORY
      .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
    let mut object_ref = std::ptr::null_mut();
    let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
    let finalize_callbacks_ptr = std::rc::Rc::into_raw(std::rc::Rc::new(std::cell::Cell::new(
      Box::into_raw(initial_finalize),
    )));
    napi::check_status!(
      napi::sys::napi_wrap(
        env,
        result,
        wrapped_value,
        Some(napi::bindgen_prelude::raw_finalize_unchecked::<Stream>),
        std::ptr::null_mut(),
        &mut object_ref,
      ),
      "Failed to wrap native object of class `{}`",
      "Stream"
    )?;
    napi::bindgen_prelude::Reference::<Stream>::add_ref(
      env,
      wrapped_value,
      (wrapped_value, object_ref, finalize_callbacks_ptr),
    );
    Ok(result)
  }
}
impl napi::bindgen_prelude::FromNapiRef for Stream {
  unsafe fn from_napi_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "Stream",
    )?;
    Ok(&*(wrapped_val as *const Stream))
  }
}
impl napi::bindgen_prelude::FromNapiMutRef for Stream {
  unsafe fn from_napi_mut_ref(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<&'static mut Self> {
    let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
    napi::bindgen_prelude::check_status!(
      napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val),
      "Failed to recover `{}` type from napi value",
      "Stream",
    )?;
    Ok(&mut *(wrapped_val as *mut Stream))
  }
}
impl napi::bindgen_prelude::FromNapiValue for &Stream {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::FromNapiValue for &mut Stream {
  unsafe fn from_napi_value(
    env: napi::bindgen_prelude::sys::napi_env,
    napi_val: napi::bindgen_prelude::sys::napi_value,
  ) -> napi::bindgen_prelude::Result<Self> {
    napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &Stream {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Stream\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "Stream"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "Stream"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "Stream"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Stream"),
      ))
    }
  }
}
impl napi::bindgen_prelude::ValidateNapiValue for &mut Stream {
  unsafe fn validate(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<napi::sys::napi_value> {
    if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Stream\0") {
      let mut ctor = std::ptr::null_mut();
      napi::check_status!(
        napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
        "Failed to get constructor reference of class `{}`",
        "Stream"
      )?;
      let mut is_instance_of = false;
      napi::check_status!(
        napi::sys::napi_instanceof(env, napi_val, ctor, &mut is_instance_of),
        "Failed to get external value of class `{}`",
        "Stream"
      )?;
      if is_instance_of {
        Ok(std::ptr::null_mut())
      } else {
        Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Value is not instanceof class `{}`", "Stream"),
        ))
      }
    } else {
      Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Failed to get constructor of class `{}`", "Stream"),
      ))
    }
  }
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
mod __napi_helper__Stream {
  use super::*;
  use std::ptr;
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__Stream_struct_29() {
    napi::__private::register_class("Stream", None, "Stream\0", vec![]);
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__Stream_struct_29() {
    napi::__private::register_class("Stream", None, "Stream\0", vec![]);
  }
}
impl Stream {
  pub fn new(send: quinn::SendStream, recv: quinn::RecvStream) -> Self {
    Self { send, recv }
  }
  pub fn id(&self) -> String {
    self.send.id().index().to_string()
  }
  pub async unsafe fn write(&mut self, data: Uint8Array) -> Result<()> {
    self.send.write_all(&data).await.map_err(to_err)
  }
  pub async unsafe fn read(&mut self, mut buf: Uint8Array) -> Result<Option<u32>> {
    let chunk = self.recv.read(buf.as_mut()).await.map_err(to_err)?;
    match chunk {
      Some(len) => Ok(Some(len as u32)),
      None => Ok(None),
    }
  }
  pub fn finish_write(&mut self) {
    let _ = self.send.finish();
  }
  pub fn reset_write(&mut self) {
    let _ = self.send.reset(0u8.into());
  }
  pub fn stop_read(&mut self) {
    let _ = self.recv.stop(0u8.into());
  }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod __napi_impl_helper__Stream__4 {
  use super::*;
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__id(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Stream>()? };
          let this: &Stream = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.id() };
            <String as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__write(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<1usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
          impl NapiRefContainer {
            fn drop(self, env: napi::sys::napi_env) {
              for r in self.0.into_iter() {
                assert_eq!(
                  unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
                assert_eq!(
                  unsafe { napi::sys::napi_delete_reference(env, r) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
              }
            }
          }
          unsafe impl Send for NapiRefContainer {}
          unsafe impl Sync for NapiRefContainer {}
          let _make_ref = |a: ::std::ptr::NonNull<napi::bindgen_prelude::sys::napi_value__>| {
            let mut node_ref = ::std::mem::MaybeUninit::uninit();
            napi::bindgen_prelude::check_status!(
              unsafe {
                napi::bindgen_prelude::sys::napi_create_reference(
                  env,
                  a.as_ptr(),
                  1,
                  node_ref.as_mut_ptr(),
                )
              },
              "failed to create napi ref"
            )?;
            Ok::<napi::sys::napi_ref, napi::Error>(unsafe { node_ref.assume_init() })
          };
          let mut _args_array =
            [::std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_ref__>(); 1usize];
          let mut _arg_write_index = 0;
          _args_array[_arg_write_index] =
            _make_ref(::std::ptr::NonNull::new(cb.this).ok_or_else(|| {
              napi::Error::new(
                napi::Status::InvalidArg,
                "referenced ptr is null".to_owned(),
              )
            })?)?;
          _arg_write_index += 1;
          #[cfg(debug_assert)]
          {
            for a in &_args_array {
              assert!(!a.is_null(), "failed to initialize napi ref");
            }
          }
          let _args_ref = NapiRefContainer(_args_array);
          let this_ptr = unsafe { cb.unwrap_raw::<Stream>()? };
          let this: &mut Stream = Box::leak(Box::from_raw(this_ptr));
          let arg0 = {
            <Uint8Array as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
              env,
              cb.get_arg(0usize),
            )?
          };
          napi::bindgen_prelude::execute_tokio_future(
            env,
            async move { this.write(arg0).await },
            move |env, _ret| {
              _args_ref.drop(env);
              <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
            },
          )
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__read(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<1usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
          impl NapiRefContainer {
            fn drop(self, env: napi::sys::napi_env) {
              for r in self.0.into_iter() {
                assert_eq!(
                  unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
                assert_eq!(
                  unsafe { napi::sys::napi_delete_reference(env, r) },
                  napi::sys::Status::napi_ok,
                  "failed to delete napi ref"
                );
              }
            }
          }
          unsafe impl Send for NapiRefContainer {}
          unsafe impl Sync for NapiRefContainer {}
          let _make_ref = |a: ::std::ptr::NonNull<napi::bindgen_prelude::sys::napi_value__>| {
            let mut node_ref = ::std::mem::MaybeUninit::uninit();
            napi::bindgen_prelude::check_status!(
              unsafe {
                napi::bindgen_prelude::sys::napi_create_reference(
                  env,
                  a.as_ptr(),
                  1,
                  node_ref.as_mut_ptr(),
                )
              },
              "failed to create napi ref"
            )?;
            Ok::<napi::sys::napi_ref, napi::Error>(unsafe { node_ref.assume_init() })
          };
          let mut _args_array =
            [::std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_ref__>(); 1usize];
          let mut _arg_write_index = 0;
          _args_array[_arg_write_index] =
            _make_ref(::std::ptr::NonNull::new(cb.this).ok_or_else(|| {
              napi::Error::new(
                napi::Status::InvalidArg,
                "referenced ptr is null".to_owned(),
              )
            })?)?;
          _arg_write_index += 1;
          #[cfg(debug_assert)]
          {
            for a in &_args_array {
              assert!(!a.is_null(), "failed to initialize napi ref");
            }
          }
          let _args_ref = NapiRefContainer(_args_array);
          let this_ptr = unsafe { cb.unwrap_raw::<Stream>()? };
          let this: &mut Stream = Box::leak(Box::from_raw(this_ptr));
          let arg0 = {
            <Uint8Array as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
              env,
              cb.get_arg(0usize),
            )?
          };
          napi::bindgen_prelude::execute_tokio_future(
            env,
            async move { this.read(arg0).await },
            move |env, _ret| {
              _args_ref.drop(env);
              <Option<u32> as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
            },
          )
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__finish_write(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Stream>()? };
          let this: &mut Stream = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.finish_write() };
            <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__reset_write(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Stream>()? };
          let this: &mut Stream = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.reset_write() };
            <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  extern "C" fn __napi__stop_read(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
  ) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
      napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
        .and_then(|mut cb| {
          let this_ptr = unsafe { cb.unwrap_raw::<Stream>()? };
          let this: &mut Stream = Box::leak(Box::from_raw(this_ptr));
          napi::bindgen_prelude::within_runtime_if_available(move || {
            let _ret = { this.stop_read() };
            <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
          })
        })
        .unwrap_or_else(|e| {
          napi::bindgen_prelude::JsError::from(e).throw_into(env);
          std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
        })
    }
  }
  #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
  #[napi::bindgen_prelude::ctor]
  fn __napi_register__Stream_impl_36() {
    napi::__private::register_class(
      "Stream",
      None,
      "Stream\0",
      vec![
        napi::bindgen_prelude::Property::new("finishWrite")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__finish_write),
        napi::bindgen_prelude::Property::new("id")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__id),
        napi::bindgen_prelude::Property::new("read")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__read),
        napi::bindgen_prelude::Property::new("resetWrite")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__reset_write),
        napi::bindgen_prelude::Property::new("stopRead")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__stop_read),
        napi::bindgen_prelude::Property::new("write")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__write),
      ],
    );
  }
  #[cfg(all(not(test), not(feature = "noop"), target_family = "wasm"))]
  #[no_mangle]
  extern "C" fn __napi_register__Stream_impl_36() {
    napi::__private::register_class(
      "Stream",
      None,
      "Stream\0",
      vec![
        napi::bindgen_prelude::Property::new("finishWrite")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__finish_write),
        napi::bindgen_prelude::Property::new("id")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__id),
        napi::bindgen_prelude::Property::new("read")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__read),
        napi::bindgen_prelude::Property::new("resetWrite")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__reset_write),
        napi::bindgen_prelude::Property::new("stopRead")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__stop_read),
        napi::bindgen_prelude::Property::new("write")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__write),
      ],
    );
  }
}
