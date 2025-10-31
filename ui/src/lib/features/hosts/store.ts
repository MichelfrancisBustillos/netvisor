import { derived, get, writable, type Readable } from 'svelte/store';
import type { AllInterfaces, Host, HostWithServicesRequest, Interface, Port } from './types/base';
import { api } from '../../shared/utils/api';
import { pushSuccess } from '$lib/shared/stores/feedback';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';
import { isContainerSubnet } from '../subnets/store';
import { getBindingFromId, getBindingDisplayName } from '../services/store';
import { currentNetwork } from '../networks/store';

export const hosts = writable<Host[]>([]);
export const polling = writable(false);

export async function getHosts() {
	return await api.request<Host[]>(
		`/hosts?network_id=${get(currentNetwork).id}`,
		hosts,
		(hosts) => hosts,
		{ method: 'GET' }
	);
}

export async function createHost(data: HostWithServicesRequest) {
	const result = await api.request<HostWithServicesRequest, Host[]>(
		'/hosts',
		hosts,
		({ host }, current) => [...current, host],
		{ method: 'POST', body: JSON.stringify(data) }
	);

	return result;
}

export async function updateHost(data: HostWithServicesRequest) {
	return await api.request<Host, Host[]>(
		`/hosts`,
		hosts,
		(updatedHost, current) => {
			return current.map((n) => (n.id === data.host.id ? updatedHost : n));
		},
		{ method: 'PUT', body: JSON.stringify(data) }
	);
}

export async function deleteHost(id: string) {
	return await api.request<void, Host[]>(
		`/hosts/${id}`,
		hosts,
		(_, current) => current.filter((g) => g.id !== id),
		{ method: 'DELETE' }
	);
}

export async function consolidateHosts(destination_host_id: string, other_host_id: string) {
	const other_host_name = get(getHostFromId(other_host_id))?.name;

	return await api.request<Host, Host[]>(
		`/hosts/${destination_host_id}/consolidate/${other_host_id}`,
		hosts,
		(updatedHost, current) => {
			current = current.filter((g) => g.id !== other_host_id);
			pushSuccess(`Consolidated host "${other_host_name}" into host "${updatedHost.name}"`);
			return current.map((h) => (h.id == destination_host_id ? updatedHost : h));
		},
		{ method: 'PUT' }
	);
}

export function createEmptyHostFormData(): Host {
	return {
		id: uuidv4Sentinel,
		created_at: utcTimeZoneSentinel,
		updated_at: utcTimeZoneSentinel,
		name: '',
		description: '',
		hostname: '',
		target: {
			type: 'None'
		},
		services: [],
		interfaces: [],
		ports: [],
		source: {
			type: 'Manual'
		},
		virtualization: null,
		network_id: get(currentNetwork).id,
		hidden: false
	};
}

export function getHostTargetString(host: Host): Readable<string> {
	return derived(
		[getBindingFromId(host.target.type === 'ServiceBinding' ? host.target.config : '')],
		([$binding]) => {
			switch (host.target.type) {
				case 'ServiceBinding': {
					if ($binding) {
						return get(getBindingDisplayName($binding));
					}
					return 'Unknown Binding';
				}
				case 'None': {
					return 'None';
				}
				case 'Hostname': {
					if (host.hostname.length > 0) return host.hostname;
					return 'Unknown Binding';
				}
			}
		}
	);
}

export function formatInterface(i: Interface | AllInterfaces): string {
	if (i.id == null) return i.name;
	return get(isContainerSubnet(i.subnet_id))
		? i.name
		: (i.name ? i.name + ': ' : '') + i.ip_address;
}

export function getHostFromId(id: string): Readable<Host | null> {
	return derived([hosts], ([$hosts]) => {
		return $hosts.find((h) => h.id == id) || null;
	});
}

export function getInterfaceFromId(id: string): Readable<Interface | null> {
	return derived([hosts], ([$hosts]) => {
		for (const host of $hosts) {
			const iface = host.interfaces.find((i) => i.id == id);
			if (iface) {
				return iface;
			}
		}
		return null;
	});
}

export function getPortFromId(id: string): Readable<Port | null> {
	return derived([hosts], ([$hosts]) => {
		for (const host of $hosts) {
			const port = host.ports.find((i) => i.id == id);
			if (port) {
				return port;
			}
		}
		return null;
	});
}
