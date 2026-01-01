-- Omakure Azure widget (hello world example).
-- This widget prints the logged-in user and active subscription via Azure CLI.

local function run(cmd)
  local pipe = io.popen(cmd)
  if not pipe then
    return nil
  end
  local out = pipe:read("*a") or ""
  pipe:close()
  return out
end

local function run_status(cmd)
  local pipe = io.popen(cmd .. " 2>/dev/null")
  if not pipe then
    return "", false
  end
  local out = pipe:read("*a") or ""
  local ok, _, status = pipe:close()
  local success = ok == true or status == 0
  return out, success
end

local function split_tsv(line)
  local parts = {}
  for part in string.gmatch(line, "[^\t\r\n]+") do
    table.insert(parts, part)
  end
  return parts
end

local function trim(value)
  if not value then
    return ""
  end
  return (value:gsub("^%s+", ""):gsub("%s+$", ""))
end

local function extract_domain(user)
  if not user then
    return "<unknown>"
  end
  local domain = user:match("@([^%s]+)$")
  if domain and domain ~= "" then
    return domain
  end
  return "<unknown>"
end

local function read_tsv(cmd)
  local out, ok = run_status(cmd)
  if not ok then
    return ""
  end
  return trim(out)
end

local function base64_decode(data)
  local alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
  local lookup = {}
  for i = 1, #alphabet do
    lookup[alphabet:sub(i, i)] = i - 1
  end

  local result = {}
  local buffer = 0
  local bits = 0
  for i = 1, #data do
    local c = data:sub(i, i)
    if c ~= "=" then
      local val = lookup[c]
      if val then
        buffer = buffer * 64 + val
        bits = bits + 6
        if bits >= 8 then
          bits = bits - 8
          local byte = math.floor(buffer / (2 ^ bits)) % 256
          table.insert(result, string.char(byte))
        end
      end
    end
  end
  return table.concat(result)
end

local function extract_oid_from_token()
  local token = read_tsv('az account get-access-token --query accessToken -o tsv')
  if token == "" then
    return ""
  end
  local payload = token:match("^[^%.]+%.([^%.]+)%.[^%.]+$")
  if not payload then
    return ""
  end
  payload = payload:gsub("-", "+"):gsub("_", "/")
  local pad = #payload % 4
  if pad > 0 then
    payload = payload .. string.rep("=", 4 - pad)
  end
  local decoded = base64_decode(payload)
  local oid = decoded:match('"oid"%s*:%s*"([^"]+)"')
  if oid then
    return oid
  end
  return ""
end

local function lookup_assignee_id(user, account_type)
  if account_type == "user" then
    local assignee_id = read_tsv('az ad signed-in-user show --query id -o tsv')
    if assignee_id ~= "" then
      return assignee_id
    end
  end
  if account_type == "servicePrincipal" then
    local assignee_id = read_tsv('az ad sp show --id "' .. user .. '" --query id -o tsv')
    if assignee_id ~= "" then
      return assignee_id
    end
  end
  return extract_oid_from_token()
end

local function split_lines(value)
  local lines = {}
  for line in string.gmatch(value or "", "[^\r\n]+") do
    table.insert(lines, trim(line))
  end
  return lines
end

local function dedupe(items)
  local seen = {}
  local out = {}
  for _, item in ipairs(items) do
    if item ~= "" and not seen[item] then
      seen[item] = true
      table.insert(out, item)
    end
  end
  return out
end

local function first_non_empty(cmds)
  local had_success = false
  for _, cmd in ipairs(cmds) do
    local out, ok = run_status(cmd)
    if ok then
      had_success = true
      local trimmed = trim(out)
      if trimmed ~= "" then
        return trimmed, true
      end
    end
  end
  if had_success then
    return "", true
  end
  return "", false
end

local title = "Azure"

local output = run(
  'az account show --only-show-errors --query "[tenantId, user.name, user.type, name, id, state, environmentName]" -o tsv'
)
if not output or output:gsub("%s+", "") == "" then
  return {
    title = title,
    lines = {
      "Azure CLI not ready. Run `az login`."
    }
  }
end

local parts = split_tsv(output)
local tenant_id = parts[1] or "<unknown>"
local user = parts[2] or "<unknown>"
local account_type = parts[3] or "<unknown>"
local sub_name = parts[4] or "<unknown>"
local sub_id = parts[5] or "<unknown>"
local sub_state = parts[6] or "<unknown>"
local cloud_env = parts[7] or "<unknown>"
local domain = extract_domain(user)

local default_location = "<not set>"
local location_output = read_tsv(
  'az configure -l --query "[?name==\'location\'].value | [0]" -o tsv'
)
if location_output ~= "" then
  default_location = location_output
end

local roles = "<unknown>"
if user ~= "<unknown>" and sub_id ~= "<unknown>" then
  local assignee_id = lookup_assignee_id(user, account_type)
  local assignee_flag = '--assignee'
  local assignee_value = user
  if assignee_id ~= "" then
    assignee_flag = '--assignee-object-id'
    assignee_value = assignee_id
  end

  local base = 'az role assignment list --only-show-errors '
    .. assignee_flag .. ' "' .. assignee_value .. '" '
    .. '--scope "/subscriptions/' .. sub_id .. '" '
    .. '--all --query "[].roleDefinitionName" -o tsv'
  local role_output, role_ok = first_non_empty({
    base .. ' --include-groups --include-inherited',
    base .. ' --include-inherited',
    base
  })

  if role_ok and role_output == "" and assignee_id ~= "" then
    local fallback_cmd = "az role assignment list --only-show-errors "
      .. '--scope "/subscriptions/' .. sub_id .. '" '
      .. '--include-inherited --all '
      .. '--query "[?principalId==\'' .. assignee_id .. '\'].roleDefinitionName" -o tsv'
    local fallback_output = read_tsv(fallback_cmd)
    if fallback_output ~= "" then
      role_output = fallback_output
      role_ok = true
    end
  end

  if role_ok then
    local role_lines = split_lines(role_output)
    local unique = dedupe(role_lines)
    if #unique > 0 then
      roles = table.concat(unique, ", ")
    else
      roles = "<none>"
    end
  else
    roles = "<unavailable>"
  end
end

return {
  title = title,
  lines = {
    "Domain: " .. domain,
    "Tenant ID: " .. tenant_id,
    "User: " .. user,
    "Account type: " .. account_type,
    "Cloud environment: " .. cloud_env,
    "Subscription: " .. sub_name .. " (" .. sub_id .. ")",
    "Subscription state: " .. sub_state,
    "Default location: " .. default_location,
    "Roles: " .. roles
  }
}
