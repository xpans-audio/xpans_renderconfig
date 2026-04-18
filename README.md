# xpans RenderConfig
Serializable and deserializable data structures for configuring spatial
audio rendering modes in the xpans Ecosystem

[![Crates.io Version](https://img.shields.io/crates/v/xpans_renderconfig)](https://crates.io/crates/xpans_renderconfig)
[![docs.rs](https://img.shields.io/docsrs/xpans_renderconfig)](https://docs.rs/xpans_renderconfig/0.1.0/xpans_renderconfig/)

## Example
Here's an example of how a headphone configuration would look in JSON:

```json
{
  "mode": "headphones",
  "config": {
    "pan_law": "sine",
    "max_itd_nanos": 650000,
    "distance_curve": "exponential",
    "distance_effect": 0.5,
    "min_distance": 0.1,
    "max_distance": 1.73
  }
}
```
