schema Configuration:
  api: API = API {}
  gateway: Gateway = Gateway {}

schema API:
  port: int = 8150
  check:
    1 <= port <= 65535, "API Port not in range 1 to 65535"

schema Gateway:
  port: int = 80
  check:
    1 <= port <= 65535, "Gatewayport Port not in range 1 to 65535"
