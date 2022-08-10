from setuptools import setup
import os
from base64 import b64decode

# required arguments

PLUGIN_NAME = os.getenv('PLUGIN_NAME')
config_data = os.getenvb(b'RCLONE_CONFIG')

if not PLUGIN_NAME:
    raise ValueError('PLUGIN_NAME is not set.')
if not config_data:
    raise ValueError('RCLONE_CONFIG is not set.')

# optional arguments, defaults are defined in Dockerfile

PLUGIN_DESCRIPTION = os.getenv('PLUGIN_DESCRIPTION')
PLUGIN_URL = os.getenv('PLUGIN_URL')
PLUGIN_AUTHOR = os.getenv('PLUGIN_AUTHOR')


RCLONE_CONFIG_FNAME = 'rclone.config'

with open(RCLONE_CONFIG_FNAME, 'wb') as f:
    f.write(b64decode(config_data))


setup(
    name=PLUGIN_NAME,
    version='1.0.0',
    description=PLUGIN_DESCRIPTION,
    author=PLUGIN_AUTHOR,
    url=PLUGIN_URL,
    py_modules=['chrclone'],
    install_requires=['chris_plugin'],
    license='MIT',
    entry_points={
        'console_scripts': [
            'chrclone = chrclone:main'
        ]
    },
    package_data={
        PLUGIN_NAME: [RCLONE_CONFIG_FNAME]
    },
    extras_require={
        'none': [],
        'dev': [
            'pytest~=7.1',
            'pytest-mock~=3.8'
        ]
    },
)
