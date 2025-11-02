const { injectable } = require('@theia/core/shared/inversify');
const { CommandContribution, CommandRegistry, MenuContribution, MenuModelRegistry } = require('@theia/core/lib/common');
const { AbstractViewContribution } = require('@theia/core/lib/browser');
const { OOPVisualizerWidget } = require('./oop-visualizer-widget');

const OOPVisualizerCommand = {
    id: 'oop-visualizer.open',
    label: 'Open OOP Sphere Visualizer'
};

class OOPVisualizerContribution extends AbstractViewContribution {
    constructor() {
        super({
            widgetId: OOPVisualizerWidget.ID,
            widgetName: OOPVisualizerWidget.LABEL,
            defaultWidgetOptions: { area: 'right' }
        });
    }

    registerCommands(registry) {
        registry.registerCommand(OOPVisualizerCommand, {
            execute: () => this.openView({ activate: true, reveal: true })
        });
    }

    registerMenus(menus) {
        menus.registerMenuAction('1_view', {
            commandId: OOPVisualizerCommand.id,
            label: 'OOP Sphere Visualizer'
        });
    }
}

module.exports = { OOPVisualizerContribution, OOPVisualizerCommand };