import { writable, get } from 'svelte/store';
import { api } from '../utils/api';
import {
	createColorHelper,
	createIconComponent,
	createLogoIconComponent,
	createStyle,
	type ColorStyle
} from '../utils/styling';

export interface TypeMetadata<TMetadata = Record<string, unknown>> {
	id: string;
	name: string;
	description: string;
	category: string;
	icon: string;
	color: string;
	metadata: TMetadata;
}

export interface EntityMetadata {
	id: string;
	color: string;
	icon: string;
}

export interface MetadataRegistry {
	service_definitions: TypeMetadata<ServicedDefinitionMetadata>[];
	subnet_types: TypeMetadata<SubnetTypeMetadata>[];
	edge_types: TypeMetadata<EdgeTypeMetadata>[];
	group_types: TypeMetadata<GroupTypeMetadata>[];
	entities: EntityMetadata[];
	ports: TypeMetadata<PortTypeMetadata>[];
}

export interface ServicedDefinitionMetadata {
	can_be_added: boolean;
	manages_virtualization: 'vms' | 'containers';
	logo_source: 'dashboard_icons' | 'simple_icons' | 'vector_zone_icons' | 'static_file_icon' | null;
}

function isValidLogoSource(
	value: unknown
): value is 'dashboard_icons' | 'simple_icons' | 'vector_zone_icons' {
	return (
		value === 'dashboard_icons' ||
		value === 'simple_icons' ||
		value === 'vector_zone_icons' ||
		value === 'static_file_icon'
	);
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface SubnetTypeMetadata {}

export interface EdgeTypeMetadata {
	is_dashed: boolean;
	has_start_marker: boolean;
	has_end_marker: boolean;
	edge_style: 'straight' | 'smoothstep' | 'bezier' | 'simplebezier' | 'step';
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface GroupTypeMetadata {}

export interface PortTypeMetadata {
	is_management: boolean;
	is_dns: boolean;
	is_custom: boolean;
	can_be_added: boolean;
	number: number;
	protocol: 'Tcp' | 'Udp';
}

export const metadata = writable<MetadataRegistry>();

// Shared color helper functions that work for both TypeMetadata and EntityMetadata
function createSharedHelpers<T extends keyof MetadataRegistry>(category: T) {
	return {
		getColorString: (id: string | null): string => {
			const $registry = get(metadata);
			const item = $registry?.[category]?.find((item) => item.id === id);
			return item?.color || 'gray';
		},

		getColorHelper: (id: string | null): ColorStyle => {
			const $registry = get(metadata);
			const item = $registry?.[category]?.find((item) => item.id === id);
			const baseColor = item?.color || null;
			return createColorHelper(baseColor);
		},

		getIcon: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as EntityMetadata[])?.find((item) => item.id === id)?.icon ||
				'HelpCircle'
			);
		},

		getIconComponent: (id: string | null) => {
			const $registry = get(metadata);
			const item = ($registry?.[category] as EntityMetadata[])?.find((item) => item.id === id);
			const iconName = item?.icon || null;
			return createIconComponent(iconName);
		},

		getStyle: (id: string | null) => {
			const $registry = get(metadata);
			const item = ($registry?.[category] as EntityMetadata[])?.find((item) => item.id === id);
			const color = item?.color || null;
			const icon = item?.icon || null;
			return createStyle(color, icon);
		}
	};
}

// Type helpers to constrain generic types
type TypeMetadataKeys = {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	[K in keyof MetadataRegistry]: MetadataRegistry[K][number] extends TypeMetadata<any> ? K : never;
}[keyof MetadataRegistry];

type EntityMetadataKeys = {
	[K in keyof MetadataRegistry]: MetadataRegistry[K][number] extends EntityMetadata ? K : never;
}[keyof MetadataRegistry];

// Full TypeMetadata helpers (includes color methods + other methods)
function createTypeMetadataHelpers<T extends TypeMetadataKeys>(category: T) {
	const sharedHelpers = createSharedHelpers(category);

	// Extract metadata type from the registry
	type MetadataType = MetadataRegistry[T][number] extends TypeMetadata<infer M> ? M : never;

	const helpers = {
		...sharedHelpers,

		getIconComponent: (id: string | null) => {
			const $registry = get(metadata);
			const item = ($registry?.[category] as TypeMetadata<MetadataType>[])?.find(
				(item) => item.id === id
			);
			const iconName = item?.icon || null;

			if (
				item?.metadata &&
				typeof item.metadata === 'object' &&
				'logo_source' in item.metadata &&
				isValidLogoSource(item.metadata.logo_source)
			) {
				if ('logo_needs_white_background' in item.metadata) {
					return createLogoIconComponent(
						iconName,
						item.metadata.logo_source,
						!!item.metadata.logo_needs_white_background
					);
				}
				return createLogoIconComponent(iconName, item.metadata.logo_source);
			}

			return createIconComponent(iconName);
		},

		getItems: () => {
			const $registry = get(metadata);
			return $registry?.[category] as TypeMetadata<MetadataType>[];
		},

		getItem: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as TypeMetadata<MetadataType>[])?.find((item) => item.id === id) ||
				null
			);
		},

		getName: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as TypeMetadata<MetadataType>[])?.find((item) => item.id === id)
					?.name ||
				id ||
				''
			);
		},

		getDescription: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as TypeMetadata<MetadataType>[])?.find((item) => item.id === id)
					?.description || ''
			);
		},

		getCategory: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as TypeMetadata<MetadataType>[])?.find((item) => item.id === id)
					?.category || ''
			);
		},

		getMetadata: (id: string | null): MetadataType => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as TypeMetadata<MetadataType>[])?.find((item) => item.id === id)
					?.metadata || ({} as MetadataType)
			);
		}
	};

	return helpers;
}

// EntityMetadata helpers (only color methods)
function createEntityMetadataHelpers<T extends EntityMetadataKeys>(category: T) {
	const sharedHelpers = createSharedHelpers(category);

	const helpers = {
		getItems: () => {
			const $registry = get(metadata);
			return $registry?.[category] as EntityMetadata[];
		},

		getItem: (id: string | null) => {
			const $registry = get(metadata);
			return ($registry?.[category] as EntityMetadata[])?.find((item) => item.id === id) || null;
		},

		// Only include the shared color methods
		...sharedHelpers
	};

	return helpers;
}

// Create all the helpers
export const serviceDefinitions = createTypeMetadataHelpers('service_definitions');
export const subnetTypes = createTypeMetadataHelpers('subnet_types');
export const edgeTypes = createTypeMetadataHelpers('edge_types');
export const groupTypes = createTypeMetadataHelpers('group_types');
export const entities = createEntityMetadataHelpers('entities');
export const ports = createTypeMetadataHelpers('ports');

export async function getMetadata() {
	await api.request<MetadataRegistry>('/metadata', metadata, (metadata) => metadata, {
		method: 'GET'
	});
}
