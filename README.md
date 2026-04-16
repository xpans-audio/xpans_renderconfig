# xpans RenderConfig
Serializable and deserializable data structures for configuring spatial
audio rendering modes in the xpans Ecosystem

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
